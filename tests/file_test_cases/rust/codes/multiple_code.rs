#[derive(Debug)]
pub struct MainFoldr {
    foldr_path: String,
}

impl MainFoldr {
    pub fn generte_foldr(systim: String, procces: u32) -> MainFoldr {
        if procces == 0 {
            panic!("");
        }

        return MainFoldr { foldr_path: systim };
    }
}
