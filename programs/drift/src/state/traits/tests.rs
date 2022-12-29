mod size {
    use crate::state::events::OrderActionRecord;
    use crate::state::insurance_fund_stake::InsuranceFundStake;
    use crate::state::perp_market::PerpMarket;
    use crate::state::spot_market::{SerumV3FulfillmentConfig, SpotMarket};
    use crate::state::state::State;
    use crate::state::traits::Size;
    use crate::state::user::{User, UserStats};

    #[test]
    fn order_action_records() {
        let expected_size = std::mem::size_of::<OrderActionRecord>() + 8;
        let actual_size = OrderActionRecord::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn perp_market() {
        let expected_size = std::mem::size_of::<PerpMarket>() + 8;
        let actual_size = PerpMarket::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn spot_market() {
        let expected_size = std::mem::size_of::<SpotMarket>() + 8;
        let actual_size = SpotMarket::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn serum_config() {
        let expected_size = std::mem::size_of::<SerumV3FulfillmentConfig>() + 8;
        let actual_size = SerumV3FulfillmentConfig::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn state() {
        let expected_size = std::mem::size_of::<State>() + 8;
        let actual_size = State::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn user() {
        let expected_size = std::mem::size_of::<User>() + 8;
        let actual_size = User::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn user_stats() {
        let expected_size = std::mem::size_of::<UserStats>() + 8;
        let actual_size = UserStats::SIZE;
        assert_eq!(actual_size, expected_size);
    }

    #[test]
    fn insurance_fund_stake() {
        let expected_size = std::mem::size_of::<InsuranceFundStake>() + 8;
        let actual_size = InsuranceFundStake::SIZE;
        assert_eq!(actual_size, expected_size);
    }
}
