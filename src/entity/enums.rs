use std::fmt::Display;
use std::str::FromStr;

#[derive(Eq, PartialEq, Debug, Default)]
pub enum OrderPaymentStatus {
    #[default]
    Unpaid,
    Paid,
    Cancelled,
}

impl OrderPaymentStatus {
    pub fn new(status: &str) -> OrderPaymentStatus {
        match status {
            "unpaid" => OrderPaymentStatus::Unpaid,
            "paid" => OrderPaymentStatus::Paid,
            "cancelled" => OrderPaymentStatus::Cancelled,
            _ => OrderPaymentStatus::Unpaid,
        }
    }
}

impl FromStr for OrderPaymentStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrderPaymentStatus::new(s))
    }
}

impl Display for OrderPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OrderPaymentStatus::Unpaid => "unpaid",
            OrderPaymentStatus::Paid => "paid",
            OrderPaymentStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum OrderShippingStatus {
    #[default]
    Pending,
    Shipped,
    PartialDelivered,
    Delivered,
    Completed,
}

impl OrderShippingStatus {
    pub fn new(status: &str) -> OrderShippingStatus {
        match status {
            "pending" => OrderShippingStatus::Pending,
            "shipped" => OrderShippingStatus::Shipped,
            "partial_delivered" => OrderShippingStatus::PartialDelivered,
            "delivered" => OrderShippingStatus::Delivered,
            "completed" => OrderShippingStatus::Completed,
            _ => OrderShippingStatus::Pending,
        }
    }
}

impl FromStr for OrderShippingStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OrderShippingStatus::new(s))
    }
}

impl Display for OrderShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            OrderShippingStatus::Pending => "pending",
            OrderShippingStatus::Shipped => "shipped",
            OrderShippingStatus::PartialDelivered => "partial_delivered",
            OrderShippingStatus::Delivered => "delivered",
            OrderShippingStatus::Completed => "completed",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum PriceInquiryStatus {
    #[default]
    Pending,
    Quoted,
    Closed,
}

impl PriceInquiryStatus {
    pub fn new(status: &str) -> PriceInquiryStatus {
        match status {
            "pending" => PriceInquiryStatus::Pending,
            "quoted" => PriceInquiryStatus::Quoted,
            "closed" => PriceInquiryStatus::Closed,
            _ => PriceInquiryStatus::Pending,
        }
    }
}

impl FromStr for PriceInquiryStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PriceInquiryStatus::new(s))
    }
}

impl Display for PriceInquiryStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PriceInquiryStatus::Pending => "pending",
            PriceInquiryStatus::Quoted => "quoted",
            PriceInquiryStatus::Closed => "closed",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum PurchaseOrderStatus {
    #[default]
    Pending,
    Received,
    PartialReceived,
    Cancelled,
    Completed,
}

impl PurchaseOrderStatus {
    pub fn new(status: &str) -> PurchaseOrderStatus {
        match status {
            "pending" => PurchaseOrderStatus::Pending,
            "received" => PurchaseOrderStatus::Received,
            "partial_received" => PurchaseOrderStatus::PartialReceived,
            "cancelled" => PurchaseOrderStatus::Cancelled,
            "completed" => PurchaseOrderStatus::Completed,
            _ => PurchaseOrderStatus::Pending,
        }
    }
}

impl FromStr for PurchaseOrderStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PurchaseOrderStatus::new(s))
    }
}

impl Display for PurchaseOrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            PurchaseOrderStatus::Pending => "pending",
            PurchaseOrderStatus::Received => "received",
            PurchaseOrderStatus::PartialReceived => "partial_received",
            PurchaseOrderStatus::Cancelled => "cancelled",
            PurchaseOrderStatus::Completed => "completed",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum CustomerStatus {
    #[default]
    Active,
    Cancelled,
    Banned,
}

impl CustomerStatus {
    pub fn new(status: &str) -> CustomerStatus {
        match status {
            "active" => CustomerStatus::Active,
            "cancelled" => CustomerStatus::Cancelled,
            "banned" => CustomerStatus::Banned,
            _ => CustomerStatus::Active,
        }
    }
}

impl FromStr for CustomerStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CustomerStatus::new(s))
    }
}

impl Display for CustomerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            CustomerStatus::Active => "active",
            CustomerStatus::Cancelled => "cancelled",
            CustomerStatus::Banned => "banned",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum AdminStatus {
    #[default]
    Active,
    Cancelled,
}

impl AdminStatus {
    pub fn new(status: &str) -> AdminStatus {
        match status {
            "active" => AdminStatus::Active,
            "cancelled" => AdminStatus::Cancelled,
            _ => AdminStatus::Active,
        }
    }
}

impl FromStr for AdminStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AdminStatus::new(s))
    }
}

impl Display for AdminStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AdminStatus::Active => "active",
            AdminStatus::Cancelled => "cancelled",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default, Ord, PartialOrd)]
pub enum AdminRole {
    #[default]
    Staff,
    Admin,
}

impl AdminRole {
    pub fn new(role: &str) -> AdminRole {
        match role {
            "staff" => AdminRole::Staff,
            "admin" => AdminRole::Admin,
            _ => AdminRole::Staff,
        }
    }
}

impl FromStr for AdminRole {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AdminRole::new(s))
    }
}

impl Display for AdminRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AdminRole::Staff => "staff",
            AdminRole::Admin => "admin",
        };
        write!(f, "{}", str)
    }
}

#[derive(Eq, PartialEq, Debug, Default)]
pub enum UserStatus {
    #[default]
    Active,
    Cancelled,
    Banned,
}

impl UserStatus {
    pub fn new(status: &str) -> UserStatus {
        match status {
            "active" => UserStatus::Active,
            "cancelled" => UserStatus::Cancelled,
            "banned" => UserStatus::Banned,
            _ => UserStatus::Active,
        }
    }
}

impl FromStr for UserStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(UserStatus::new(s))
    }
}

impl Display for UserStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            UserStatus::Active => "active",
            UserStatus::Cancelled => "cancelled",
            UserStatus::Banned => "banned",
        };
        write!(f, "{}", str)
    }
}
