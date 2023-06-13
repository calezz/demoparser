use memmap2::Mmap;
use memmap2::MmapOptions;
use parser::parser_settings::create_huffman_lookup_table;
use parser::parser_settings::Parser;
use parser::parser_settings::ParserInputs;
use parser::read_bits::Bitreader;
use parser::read_bits::DemoParserError;
use std::fs;
use std::fs::File;
use std::time::Instant;

fn main() {
    let wanted_props = vec!["CCSPlayerPawn.m_iHealth".to_owned()];
    let demo_path = "/home/laiho/Documents/demos/cs2/s2-gotv.dem";
    let a = Instant::now();
    let file = File::open(demo_path).unwrap();
    let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };
    //let bytes = fs::read(demo_path).unwrap();

    let huf = create_huffman_lookup_table();
    let dir = fs::read_dir("/home/laiho/Documents/demos/cs2/test/").unwrap();
    for path in dir {
        println!("{:?}", path.as_ref().unwrap().path());

        let file = File::open(path.unwrap().path()).unwrap();
        let mmap = unsafe { MmapOptions::new().map(&file).unwrap() };

        let settings = ParserInputs {
            bytes: &mmap,
            wanted_player_props: wanted_props.clone(),
            wanted_player_props_og_names: wanted_props.clone(),
            wanted_event: Some("bomb_planted".to_string()),
            wanted_other_props: vec![
                "CCSTeam.m_iScore".to_string(),
                "CCSTeam.m_szTeamname".to_string(),
                "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
            ],
            wanted_other_props_og_names: vec![
                "score".to_string(),
                "name".to_string(),
                "CCSGameRulesProxy.CCSGameRules.m_totalRoundsPlayed".to_string(),
            ],
            parse_ents: true,
            wanted_ticks: vec![],
            parse_projectiles: false,
            only_header: false,
            count_props: false,
            only_convars: false,
            huf: &huf,
        };

        use rayon::prelude::*;

        let before = Instant::now();

        let mut parser = Parser::new(settings.clone()).unwrap();
        let md = parser.front_demo_metadata().unwrap();
        // println!("FRONT TOOK: {:2?}", before.elapsed());

        let mut parsers = vec![];

        let before = Instant::now();
        for offset in md.fullpacket_offsets {
            let mut parser2 = Parser::new(settings.clone()).unwrap();
            parser2.ptr = offset;
            //parser2.cls_by_id = parser::parser_settings::CLSBYID::Ref(&n);
            parser2.cls_bits = parser.cls_bits.clone();
            parser2.qf_map = parser.qf_map.clone();
            parser2.controller_ids = parser.controller_ids.clone();
            parsers.push(parser2);
        }
        // println!("COPY TOOK: {:2?}", before.elapsed());

        let before = Instant::now();

        match &parser.cls_by_id {
            parser::parser_settings::CLSBYID::Normal(n) => {
                let res: Vec<Result<i32, DemoParserError>> = parsers
                    .par_iter_mut()
                    .map(|p| {
                        p.cls_by_id = parser::parser_settings::CLSBYID::Ref(&n);
                        p.start()
                    })
                    .collect();
                // println!("{:?}", res);
            }
            _ => {}
        }
        // println!("PARSING TOOK: {:2?}", before.elapsed());

        // println!("TOTAL TOOK: {:2?}", a.elapsed());
    }
}
