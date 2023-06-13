use std::sync::RwLock;
use std::time::Instant;

use super::{read_bits::DemoParserError, sendtables::Serializer};
use crate::parser_settings::ChatMessageRecord;
use crate::parser_settings::EconItem;
use crate::parser_settings::Parser;
use crate::parser_settings::PlayerEndData;
use ahash::AHashMap;
use ahash::HashSet;
use csgoproto::cstrike15_usermessages::CCSUsrMsg_EndOfMatchAllPlayersData;
use csgoproto::cstrike15_usermessages::CCSUsrMsg_SendPlayerItemDrops;
use csgoproto::demo::CDemoClassInfo;
use csgoproto::demo::CDemoFileInfo;
use csgoproto::networkbasetypes::CNETMsg_SetConVar;
use csgoproto::usermessages::CUserMessageSayText2;
use protobuf::Message;
use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;

// This file has functions for the simpler netmessages.
// Don't want to create a new file for each of these.

#[derive(Debug, Clone)]
pub struct Class {
    pub name: String,
    pub serializer: Serializer,
}

impl<'a> Parser<'a> {
    pub fn parse_class_info(
        &mut self,
        bytes: &[u8],
        global_ser: Arc<RwLock<AHashMap<String, Serializer>>>,
    ) -> Result<(), DemoParserError> {
        let before = Instant::now();
        if !self.parse_entities {
            //return Ok(AHashMap::default());
        }
        let my_bytes = bytes.to_vec();
        let can_start = self.sendtables_done.clone();

        let t = self.start.clone();

        let handle = thread::spawn(move || {
            let mut cls_by_id: [Option<Class>; 560] = [
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None, None, None,
            ];
            let msg: CDemoClassInfo = Message::parse_from_bytes(&my_bytes).unwrap();
            while !can_start.load(std::sync::atomic::Ordering::SeqCst) {
                // println!("WAITING");
            }
            println!("DONE");

            let ser = global_ser.read().unwrap();

            for class_t in &msg.classes {
                let cls_id = class_t.class_id();
                let network_name = class_t.network_name();
                cls_by_id[cls_id as usize] = Some(Class {
                    name: network_name.to_string(),
                    serializer: ser.get(network_name).unwrap().clone(),
                });
            }
            println!("E {:2?}", t.elapsed());
            cls_by_id
        });
        self.cls_by_id_handle = Some(handle);
        Ok(())
    }

    pub fn parse_item_drops(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let drops: CCSUsrMsg_SendPlayerItemDrops = Message::parse_from_bytes(&bytes).unwrap();
        for item in &drops.entity_updates {
            self.item_drops.push(EconItem {
                account_id: item.accountid,
                item_id: item.itemid,
                def_index: item.defindex,
                paint_index: item.paintindex,
                rarity: item.rarity,
                quality: item.quality,
                paint_seed: item.paintseed,
                paint_wear: item.paintwear,
                quest_id: item.questid,
                dropreason: item.dropreason,
                custom_name: item.customname.clone(),
                inventory: item.inventory,
                ent_idx: item.entindex,
                steamid: None,
            });
        }
        Ok(())
    }
    pub fn parse_chat_messages(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let chat_msg: CUserMessageSayText2 = Message::parse_from_bytes(bytes).unwrap();
        self.chat_messages.push(ChatMessageRecord {
            entity_idx: chat_msg.entityindex,
            param1: chat_msg.param1,
            param2: chat_msg.param2,
            param3: chat_msg.param3,
            param4: chat_msg.param4,
        });
        Ok(())
    }
    pub fn parse_convars(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let convar: CNETMsg_SetConVar = Message::parse_from_bytes(bytes).unwrap();
        for cv in &convar.convars {
            for var in &cv.cvars {
                self.convars
                    .insert(var.name().to_owned(), var.value().to_owned());
            }
        }
        Ok(())
    }

    pub fn parse_player_end_msg(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let end_data: CCSUsrMsg_EndOfMatchAllPlayersData =
            Message::parse_from_bytes(&bytes).unwrap();
        /*
        Todo parse "accolade", seems to be the awards at the end like "most mvps in game"
        But seems to only have integers so need to figure out what they mean
        example:

        Accolade {
            eaccolade: Some(
                21,
            ),
            value: Some(
                5100.0,
            ),
            position: Some(
                1,
            ),
        }
        */
        for player in &end_data.allplayerdata {
            self.player_end_data.push(PlayerEndData {
                name: player.name.clone(),
                steamid: player.xuid,
                team_number: player.teamnumber,
            });
            for item in &player.items {
                if item.itemid() != 0 {
                    self.skins.push(EconItem {
                        account_id: item.accountid,
                        item_id: item.itemid,
                        def_index: item.defindex,
                        paint_index: item.paintindex,
                        rarity: item.rarity,
                        quality: item.quality,
                        paint_seed: item.paintseed,
                        paint_wear: item.paintwear,
                        quest_id: item.questid,
                        dropreason: item.dropreason,
                        custom_name: item.customname.clone(),
                        inventory: item.inventory,
                        ent_idx: item.entindex,
                        steamid: player.xuid,
                    });
                }
            }
        }
        Ok(())
    }
    pub fn parse_player_stats_update(&mut self, _bytes: &[u8]) -> Result<(), DemoParserError> {
        // Only in pov demos
        // let upd: CCSUsrMsg_PlayerStatsUpdate = Message::parse_from_bytes(bytes).unwrap();
        Ok(())
    }
    pub fn parse_file_info(&mut self, bytes: &[u8]) -> Result<(), DemoParserError> {
        let _info: CDemoFileInfo = Message::parse_from_bytes(bytes).unwrap();
        Ok(())
    }
}
