use super::partial_crosslink_record::PartialCrosslinkRecord;
use super::recent_proposer_record::RecentPropserRecord;
use super::rlp::{ RlpStream, Encodable };
use super::utils::types::*;

pub struct ActiveState {
    pub height: u64,
    pub randao: Sha256Digest,
    pub ffg_voter_bitfield: Bitfield,
    pub recent_attesters: Vec<u32>, // TODO: should be u24
    pub partial_crosslinks: Vec<PartialCrosslinkRecord>,
    pub total_skip_count: u64,
    pub recent_proposers: Vec<RecentPropserRecord>
}

impl ActiveState {
    pub fn new_for_height(height: u64) -> ActiveState {
        ActiveState {
            height: height,
            randao: Sha256Digest::random(),
            ffg_voter_bitfield: Vec::new(),
            recent_attesters: Vec::new(),
            partial_crosslinks: Vec::new(),
            total_skip_count: 0,
            recent_proposers: Vec::new()
        }
    }

    pub fn num_recent_proposers(&self) -> usize {
        self.recent_proposers.len()
    }
    
    pub fn num_recent_attesters(&self) -> usize {
        self.recent_attesters.len()
    }
}

/*
 * RLP Encoding
 */
impl Encodable for ActiveState {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.height);
        s.append(&self.randao);
        s.append(&self.ffg_voter_bitfield);
        s.append_list(&self.recent_attesters);
        s.append_list(&self.partial_crosslinks);
        s.append(&self.total_skip_count);
        s.append_list(&self.recent_proposers);
    }
}


#[cfg(test)]
mod tests {
    use super::super::rlp;
    use super::*;

    #[test]
    fn test_new_for_height() {
        let h = 1;
        let a = ActiveState::new_for_height(h);
        assert_eq!(a.height, h);
    }

    #[test]
    fn test_num_recent_proposers() {
        let mut a = ActiveState::new_for_height(1);
        for _ in 1..5 {
            a.recent_proposers.push(RecentPropserRecord::new(
                    1, 
                    Sha256Digest::random(), 
                    2));
        }
        assert_eq!(a.num_recent_proposers(), 4)
    }
    
    #[test]
    fn test_num_recent_attesters() {
        let mut a = ActiveState::new_for_height(1);
        for _ in 1..5 {
            a.recent_attesters.push(1);
        }
        assert_eq!(a.num_recent_attesters(), 4)
    }

    #[test]
    fn test_serialization() {
        let a = ActiveState {
            height: 100,
            randao: Sha256Digest::zero(),
            ffg_voter_bitfield: Vec::new(),
            recent_attesters: Vec::new(),
            partial_crosslinks: Vec::new(),
            total_skip_count: 99,
            recent_proposers: Vec::new()
        };
        let e = rlp::encode(&a);
        assert_eq!(e.len(), 39);
        assert_eq!(e[0], 100);
        assert_eq!(e[1], 160);
        assert_eq!(e[2..34], [0; 32]);
        assert_eq!(e[34], 128);
        assert_eq!(e[35], 192);
        assert_eq!(e[36], 192);
        assert_eq!(e[37], 99);
        assert_eq!(e[38], 192);
    }
}