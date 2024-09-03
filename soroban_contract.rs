#![no_std]
use soroban_sdk::{contractimpl, contracttype, Env, Symbol, Vec, Address};

// account info
#[contracttype]
pub struct Account {
    pub role: Role,
    pub address: Address,
}

// roles
#[contracttype]
pub enum Role {
    Creator,
    Maker,
    Shopper,
}

// Struct for storing job information
#[contracttype]
pub struct Job {
    pub id: Symbol,
    pub creator: Address,
    pub maker: Address,
    pub price: i128,
    pub is_completed: bool,
}

// Struct for NFT metadata
#[contracttype]
pub struct NFT {
    pub id: Symbol,
    pub owner: Address,
    pub metadata: String,
}

// Struct for storing marketplace listing
#[contracttype]
pub struct Listing {
    pub nft_id: Symbol,
    pub price: i128,
    pub status: ListingStatus,
}

// Enum to define listing status
#[contracttype]
pub enum ListingStatus {
    Listed,
    Sold,
}

pub struct JobContract;

#[contractimpl]
impl JobContract {
    // Create a new account
    pub fn create_account(env: &Env, role: Role, address: Address) {
        let account = Account { role, address: address.clone() };
        env.storage().set(address, account);
    }

    // Create and store a new job
    pub fn create_job(env: &Env, id: Symbol, creator: Address, maker: Address, price: i128) {
        let job = Job {
            id: id.clone(),
            creator: creator.clone(),
            maker,
            price,
            is_completed: false,
        };

        // Store the job in a map (key: job id, value: job)
        env.storage().set(id, job);
    }

    // Mark a job as completed
    pub fn complete_job(env: &Env, id: Symbol) {
        let mut job: Job = env.storage().get(&id).unwrap_or_else(|| panic!("Job not found"));
        job.is_completed = true;

        // Update the job status in storage
        env.storage().set(id, job);
    }

    // Release payment to the maker
    pub fn release_payment(env: &Env, id: Symbol) {
        let job: Job = env.storage().get(&id).unwrap_or_else(|| panic!("Job not found"));

        // Ensure the job is completed
        if job.is_completed {
            // Logic to transfer payment (mock logic here)
            // In a real contract, this would involve transferring tokens
            env.emit_event("PaymentReleased", id);
        } else {
            panic!("Job not completed yet.");
        }
    }

    // Mint a new NFT
    pub fn mint_nft(env: &Env, nft_id: Symbol, owner: Address, metadata: String) {
        let nft = NFT {
            id: nft_id.clone(),
            owner: owner.clone(),
            metadata,
        };

        // Store the NFT
        env.storage().set(nft_id, nft);
    }

    // List an NFT in the marketplace
    pub fn list_nft(env: &Env, nft_id: Symbol, price: i128) {
        let listing = Listing {
            nft_id: nft_id.clone(),
            price,
            status: ListingStatus::Listed,
        };

        // Store the listing in a map (key: nft id, value: listing)
        env.storage().set(nft_id, listing);
    }

    // Buy an NFT from the marketplace
    pub fn buy_nft(env: &Env, nft_id: Symbol, buyer: Address) {
        let mut listing: Listing = env.storage().get(&nft_id).unwrap_or_else(|| panic!("Listing not found"));

        // Ensure the NFT is still listed
        if let ListingStatus::Listed = listing.status {
            // Logic to handle payment (mock logic)
            listing.status = ListingStatus::Sold;

            // Transfer NFT ownership (simplified logic)
            let mut nft: NFT = env.storage().get(&nft_id).unwrap();
            nft.owner = buyer.clone();

            // Update the listing and NFT in storage
            env.storage().set(nft_id.clone(), listing);
            env.storage().set(nft_id, nft);

            env.emit_event("NFTSold", nft_id);
        } else {
            panic!("NFT not available for purchase.");
        }
    }

    // Get job details
    pub fn get_job(env: &Env, id: Symbol) -> Job {
        env.storage().get(&id).unwrap_or_else(|| panic!("Job not found"))
    }

    // Get account details
    pub fn get_account(env: &Env, address: Address) -> Account {
        env.storage().get(&address).unwrap_or_else(|| panic!("Account not found"))
    }

    // Get NFT details
    pub fn get_nft(env: &Env, nft_id: Symbol) -> NFT {
        env.storage().get(&nft_id).unwrap_or_else(|| panic!("NFT not found"))
    }
}
