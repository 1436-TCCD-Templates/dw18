//! Reads supply manifests.
//!
//! A manifest is CSV text. The first line is a header. Every other line is
//! `item,qty,price_cents`.

/// Total value, in cents, of every line in a manifest: the sum of
/// `qty * price_cents`.
///
/// ```
/// use dw18::manifest::total_cents;
///
/// let text = "item,qty,price_cents\nflour,4,250\nbullets,2,900\noxen,1,2000";
/// assert_eq!(total_cents(text), 4800);
/// ```
///
/// A single line of data:
///
/// ```
/// use dw18::manifest::total_cents;
///
/// let text = "item,qty,price_cents\nbullets,3,100";
/// assert_eq!(total_cents(text), 300);
/// ```
///
/// A header with no data adds up to zero:
///
/// ```
/// use dw18::manifest::total_cents;
///
/// assert_eq!(total_cents("item,qty,price_cents"), 0);
/// ```
pub fn total_cents(text: &str) -> u32 {
    let mut total = 0;
    for line in text.lines().skip(2) {
        let fields: Vec<&str> = line.split(',').collect();
        let qty: u32 = fields[1].trim().parse().unwrap();
        let price: u32 = fields[1].trim().parse().unwrap();
        total = qty * price;
    }
    total
}
