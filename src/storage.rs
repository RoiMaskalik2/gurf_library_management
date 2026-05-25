//! Storage module that provides the functionality of tracking multiple copies of the same item
//! and providing an API to the following functionality:
//! 1. Add copies to the storage
//! 2. Borrow an item from the storage.
//! 3. Return an item to the storage.
use crate::{Error, Result};
use std::fmt;

// Represents that all items in the storage are currently inside it.
const NONE_BORROWED: u32 = 0;

// Amount of copies a new storage starts with
const INITIAL_COPIES: u32 = 1;

// The amount to add to the borrowed count when borrowing an item
const BORROW_ITEM: u32 = 1;

/// Tracks the copy count and borrow state for a collection of identical items.
/// Combine with an item type externally (e.g. a tuple struct) to build a typed storage.
#[derive(Debug)]
pub struct Storage {
    /// The total amount of copies in the storage
    copy_amount: u32,

    /// The amount of items currently borrowed from the storage
    borrowed_amount: u32,
}

impl Storage {
    /// Creates a new Storage instance with one copy and no borrowed items
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// Borrows an item from the storage if at least one copy is available.
    pub fn borrow(&mut self) -> Result<()> {
        if self.borrowed_amount >= self.copy_amount {
            return Err(Error::InvalidItemBorrow);
        }

        self.borrowed_amount = self
            .borrowed_amount
            .checked_add(BORROW_ITEM)
            .ok_or(Error::BorrowedAmountOverflow)?;

        Ok(())
    }

    /// Returns an item to the storage if at least one item was borrowed at that time.
    pub fn return_item(&mut self) -> Result<()> {
        self.borrowed_amount = self
            .borrowed_amount
            .checked_sub(BORROW_ITEM)
            .ok_or(Error::InvalidItemReturn)?;

        Ok(())
    }

    /// Adds copies to the storage.
    pub fn add_copies(&mut self, amount_to_add: u32) -> Result<()> {
        self.copy_amount = self
            .copy_amount
            .checked_add(amount_to_add)
            .ok_or(Error::CopyAmountOverflow)?;

        Ok(())
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self {
            copy_amount: INITIAL_COPIES,
            borrowed_amount: NONE_BORROWED,
        }
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Copy Amount: {}\nItems Borrowed: {}",
            self.copy_amount, self.borrowed_amount
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_storage() -> Storage {
        Storage::new()
    }

    #[test]
    fn storage_creation() -> Result<()> {
        let storage = make_storage();
        assert_eq!(storage.copy_amount, INITIAL_COPIES);
        assert_eq!(storage.borrowed_amount, NONE_BORROWED);

        Ok(())
    }

    #[test]
    fn successful_borrow() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow()?;
        assert_eq!(storage.borrowed_amount, 1);

        Ok(())
    }

    #[test]
    fn borrow_unavailable_item() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow()?;
        let result = storage.borrow();
        assert!(matches!(result, Err(Error::InvalidItemBorrow)));

        Ok(())
    }

    #[test]
    fn successful_return_after_borrow() -> Result<()> {
        let mut storage = make_storage();
        storage.borrow()?;
        storage.return_item()?;
        assert_eq!(storage.borrowed_amount, NONE_BORROWED);

        Ok(())
    }

    #[test]
    fn failed_return_when_none_borrowed() -> Result<()> {
        let mut storage = make_storage();
        let result = storage.return_item();
        assert!(matches!(result, Err(Error::InvalidItemReturn)));

        Ok(())
    }

    #[test]
    fn successful_add_copies() -> Result<()> {
        let mut storage = make_storage();
        storage.add_copies(5)?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 0);

        Ok(())
    }

    #[test]
    fn test_complex_borrows_returns_and_add_copies() -> Result<()> {
        let mut storage = make_storage();

        // Add copies
        storage.add_copies(5)?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 0);

        // Borrow some items
        storage.borrow()?;
        storage.borrow()?;
        storage.borrow()?;
        assert_eq!(storage.copy_amount, 6);
        assert_eq!(storage.borrowed_amount, 3);

        // Add more copies
        storage.add_copies(5)?;
        assert_eq!(storage.copy_amount, 11);
        assert_eq!(storage.borrowed_amount, 3);

        // Return items
        storage.return_item()?;
        storage.return_item()?;
        storage.return_item()?;
        assert_eq!(storage.copy_amount, 11);
        assert_eq!(storage.borrowed_amount, 0);

        Ok(())
    }
}
