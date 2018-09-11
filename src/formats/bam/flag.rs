#[derive(Clone, Copy, Debug)]
pub struct Flag(u16);

impl Flag {
    pub fn new(flag: u16) -> Flag {
        Flag(flag)
    }

    pub fn inner(&self) -> u16 {
        self.0
    }

    pub fn is_paired(&self) -> bool {
        self.0 & 0x01 != 0
    }

    pub fn is_proper_pair(&self) -> bool {
        self.0 & 0x02 != 0
    }

    pub fn is_unmapped(&self) -> bool {
        self.0 & 0x04 != 0
    }

    pub fn is_mate_unmapped(&self) -> bool {
        self.0 & 0x08 != 0
    }

    pub fn is_reverse(&self) -> bool {
        self.0 & 0x10 != 0
    }

    pub fn is_mate_reverse(&self) -> bool {
        self.0 & 0x20 != 0
    }

    pub fn is_read_1(&self) -> bool {
        self.0 & 0x40 != 0
    }

    pub fn is_read_2(&self) -> bool {
        self.0 & 0x80 != 0
    }

    pub fn is_secondary(&self) -> bool {
        self.0 & 0x0100 != 0
    }

    pub fn is_qc_fail(&self) -> bool {
        self.0 & 0x0200 != 0
    }

    pub fn is_dup(&self) -> bool {
        self.0 & 0x0400 != 0
    }

    pub fn is_supplementary(&self) -> bool {
        self.0 & 0x0800 != 0
    }
}

#[cfg(test)]
mod tests {
    use super::Flag;

    #[test]
    fn test_empty_flag() {
        let flag = Flag::new(0);

        assert!(!flag.is_paired());
        assert!(!flag.is_proper_pair());
        assert!(!flag.is_unmapped());
        assert!(!flag.is_mate_unmapped());
        assert!(!flag.is_reverse());
        assert!(!flag.is_mate_reverse());
        assert!(!flag.is_read_1());
        assert!(!flag.is_read_2());
        assert!(!flag.is_secondary());
        assert!(!flag.is_qc_fail());
        assert!(!flag.is_dup());
        assert!(!flag.is_supplementary());
    }

    #[test]
    fn test_flags() {
        assert!(Flag::new(0x01).is_paired());
        assert!(Flag::new(0x02).is_proper_pair());
        assert!(Flag::new(0x04).is_unmapped());
        assert!(Flag::new(0x08).is_mate_unmapped());
        assert!(Flag::new(0x10).is_reverse());
        assert!(Flag::new(0x20).is_mate_reverse());
        assert!(Flag::new(0x40).is_read_1());
        assert!(Flag::new(0x80).is_read_2());
        assert!(Flag::new(0x0100).is_secondary());
        assert!(Flag::new(0x0200).is_qc_fail());
        assert!(Flag::new(0x0400).is_dup());
        assert!(Flag::new(0x0800).is_supplementary());
    }
}
