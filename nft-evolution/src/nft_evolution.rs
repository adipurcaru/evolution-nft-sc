#![no_std]

elrond_wasm::imports!();


#[elrond_wasm::contract]
pub trait NftEvolution {

    #[init]
    fn init(
        &self,
        amount: BigUint,
        first_collection_identifier: &TokenIdentifier,
        second_collection_identifier: &TokenIdentifier,
        upgraded_collection_identifier: &TokenIdentifier,
        number_of_remaining_upgraded_nfts: u64,
    ) -> SCResult<()> {

        self.amount().set(&amount);
        self.set_nft_first_collection_identifier(&first_collection_identifier);
        self.set_nft_second_collection_identifier(&second_collection_identifier);
        self.set_nft_upgraded_collection_identifier(&upgraded_collection_identifier);

        self.set_number_of_remaining_upgraded_nfts(&number_of_remaining_upgraded_nfts);

        Ok(())
    }

    // endpoints

    #[payable("*")]
    #[endpoint]
    fn evolve_nft(
        &self
    ) -> SCResult<()> {

        let caller = self.blockchain().get_caller();
        let receivedTokens = self.call_value().all_esdt_transfers();
        let expected_first_collection_identifier = self.get_nft_first_collection_identifier();
        let expected_second_collection_identifier = self.get_nft_second_collection_identifier();

        let mut hasFirstCollection = false;
        let mut hasSecondCollection = false;

        require!(receivedTokens.len() == 2, "Transaction requires exactly 2 nfts!");

        for payment in receivedTokens.into_iter() {
            if payment.token_identifier == expected_first_collection_identifier {
                hasFirstCollection = true;
            }

            if payment.token_identifier == expected_second_collection_identifier {
                hasSecondCollection = true;
            }
        }

        require!(hasFirstCollection && hasSecondCollection, "You need one nft from each of the 2 collections!");

        let balance = self.blockchain().get_sc_balance(&self.get_nft_upgraded_collection_identifier(), self.get_number_of_remaining_upgraded_nfts());

        require!(balance == 1, "Nft does not exist!");

        self.send().direct(
            &caller,
            &self.get_nft_upgraded_collection_identifier(),
            self.get_number_of_remaining_upgraded_nfts(),
            &self.amount().get(),
            b"Success",
        );

        let numberOfUpgradedNfts = self.get_number_of_remaining_upgraded_nfts() - 1;

        self.set_number_of_remaining_upgraded_nfts(&numberOfUpgradedNfts);

        Ok(())
    }

    // views

    #[only_owner]
    #[payable("*")]
    #[endpoint]
    fn store_nfts(&self) -> SCResult<()> {
        let expected_collection_identifier = self.get_nft_upgraded_collection_identifier();
        let receivedTokens = self.call_value().all_esdt_transfers();

        for payment in receivedTokens.into_iter() {
            require!(payment.token_identifier == expected_collection_identifier, "Wrong collection!");
        }
        Ok(())
    }

    // storage

    #[storage_set("nftFirstCollectionIdentifier")]
    fn set_nft_first_collection_identifier(&self, esdt_token_identifier: &TokenIdentifier);

    #[view(getnftFirstCollectionIdentifier)]
    #[storage_get("nftFirstCollectionIdentifier")]
    fn get_nft_first_collection_identifier(&self) -> TokenIdentifier;

    #[storage_set("nftSecondCollectionIdentifier")]
    fn set_nft_second_collection_identifier(&self, esdt_token_identifier: &TokenIdentifier);

    #[view(getnftSecondCollectionIdentifier)]
    #[storage_get("nftSecondCollectionIdentifier")]
    fn get_nft_second_collection_identifier(&self) -> TokenIdentifier;

    #[storage_set("nftUpgradedCollectionIdentifier")]
    fn set_nft_upgraded_collection_identifier(&self, esdt_token_identifier: &TokenIdentifier);

    #[view(getnftUpgradedCollectionIdentifier)]
    #[storage_get("nftUpgradedCollectionIdentifier")]
    fn get_nft_upgraded_collection_identifier(&self) -> TokenIdentifier;

    #[view(getNumberOfRemainingUpgradedNfts)]
    #[storage_get("numberOfRemainingUpgradedNfts")]
    fn get_number_of_remaining_upgraded_nfts(&self) -> u64;

    #[storage_set("numberOfRemainingUpgradedNfts")]
    fn set_number_of_remaining_upgraded_nfts(&self, number_of_upgraded_nfts: &u64);

    #[view(getAmount)]
    #[storage_mapper("amount")]
    fn amount(&self) -> SingleValueMapper<BigUint>;

}
