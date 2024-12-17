/*
 Navicat Premium Dump SQL

 Source Server         : bookstore
 Source Server Type    : MySQL
 Source Server Version : 80040 (8.0.40)
 Source Host           : localhost:3306
 Source Schema         : bookstore

 Target Server Type    : MySQL
 Target Server Version : 80040 (8.0.40)
 File Encoding         : 65001

 Date: 17/12/2024 18:41:05
*/

SET NAMES utf8mb4;
SET FOREIGN_KEY_CHECKS = 0;

-- ----------------------------
-- Table structure for admins
-- ----------------------------
DROP TABLE IF EXISTS `admins`;
CREATE TABLE `admins` (
  `admin_id` int unsigned NOT NULL AUTO_INCREMENT,
  `admin_username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `admin_pwd` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `status` enum('active','cancelled') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'active',
  `role` enum('admin','staff') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'staff',
  PRIMARY KEY (`admin_id`),
  UNIQUE KEY `admin_username` (`admin_username`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of admins
-- ----------------------------
BEGIN;
INSERT INTO `admins` (`admin_id`, `admin_username`, `admin_pwd`, `status`, `role`) VALUES (1, 'root', 'tN88Bj4=', 'active', 'admin');
INSERT INTO `admins` (`admin_id`, `admin_username`, `admin_pwd`, `status`, `role`) VALUES (2, 'reinerina', '45h9W2VXv8K8bg==', 'active', 'admin');
COMMIT;

-- ----------------------------
-- Table structure for authed_customers
-- ----------------------------
DROP TABLE IF EXISTS `authed_customers`;
CREATE TABLE `authed_customers` (
  `customer_id` int unsigned NOT NULL,
  `token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `last_used` datetime NOT NULL,
  `is_online` tinyint unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`customer_id`),
  CONSTRAINT `authed_customer_id` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`customer_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of authed_customers
-- ----------------------------
BEGIN;
INSERT INTO `authed_customers` (`customer_id`, `token`, `last_used`, `is_online`) VALUES (1, 'K4Ld0BF/V04arb9VSWOXM0Iu8ip3pEv9M+KiAENy9Uqn38lTWWi8HBBCGKjg', '2024-12-17 16:52:52', 1);
COMMIT;

-- ----------------------------
-- Table structure for authors
-- ----------------------------
DROP TABLE IF EXISTS `authors`;
CREATE TABLE `authors` (
  `author_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`author_id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of authors
-- ----------------------------
BEGIN;
INSERT INTO `authors` (`author_id`, `name`) VALUES (1, 'Michael Downey');
INSERT INTO `authors` (`author_id`, `name`) VALUES (2, 'Patricia Yeung');
INSERT INTO `authors` (`author_id`, `name`) VALUES (3, 'Yuanshi Bu');
COMMIT;

-- ----------------------------
-- Table structure for book_authors
-- ----------------------------
DROP TABLE IF EXISTS `book_authors`;
CREATE TABLE `book_authors` (
  `book_id` int unsigned NOT NULL,
  `author_id` int unsigned NOT NULL,
  `order` tinyint NOT NULL,
  PRIMARY KEY (`book_id`,`author_id`),
  KEY `author_id` (`author_id`),
  CONSTRAINT `author_id` FOREIGN KEY (`author_id`) REFERENCES `authors` (`author_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `book_author_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of book_authors
-- ----------------------------
BEGIN;
INSERT INTO `book_authors` (`book_id`, `author_id`, `order`) VALUES (1, 1, 1);
INSERT INTO `book_authors` (`book_id`, `author_id`, `order`) VALUES (1, 2, 2);
INSERT INTO `book_authors` (`book_id`, `author_id`, `order`) VALUES (2, 3, 1);
COMMIT;

-- ----------------------------
-- Table structure for book_keywords
-- ----------------------------
DROP TABLE IF EXISTS `book_keywords`;
CREATE TABLE `book_keywords` (
  `book_id` int unsigned NOT NULL,
  `keyword_id` int unsigned NOT NULL,
  PRIMARY KEY (`book_id`,`keyword_id`) USING BTREE,
  KEY `keyword_id` (`keyword_id`),
  CONSTRAINT `keyword_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`keyword_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of book_keywords
-- ----------------------------
BEGIN;
INSERT INTO `book_keywords` (`book_id`, `keyword_id`) VALUES (1, 1);
INSERT INTO `book_keywords` (`book_id`, `keyword_id`) VALUES (1, 2);
INSERT INTO `book_keywords` (`book_id`, `keyword_id`) VALUES (2, 2);
INSERT INTO `book_keywords` (`book_id`, `keyword_id`) VALUES (2, 3);
COMMIT;

-- ----------------------------
-- Table structure for book_loactions
-- ----------------------------
DROP TABLE IF EXISTS `book_loactions`;
CREATE TABLE `book_loactions` (
  `book_id` int unsigned NOT NULL,
  `location_id` int unsigned NOT NULL,
  `quantity` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`book_id`,`location_id`) USING BTREE,
  KEY `location_id` (`location_id`),
  CONSTRAINT `book_location_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `location_id` FOREIGN KEY (`location_id`) REFERENCES `loactions` (`location_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `stk_quan_pos` CHECK ((`quantity` >= 0))
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of book_loactions
-- ----------------------------
BEGIN;
INSERT INTO `book_loactions` (`book_id`, `location_id`, `quantity`) VALUES (1, 1, 190);
COMMIT;

-- ----------------------------
-- Table structure for book_shortages
-- ----------------------------
DROP TABLE IF EXISTS `book_shortages`;
CREATE TABLE `book_shortages` (
  `shortage_item_id` int unsigned NOT NULL AUTO_INCREMENT,
  `shortage_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `supplier_id` int unsigned NOT NULL,
  `shortage` int unsigned NOT NULL,
  PRIMARY KEY (`shortage_item_id`,`shortage_id`,`book_id`,`supplier_id`) USING BTREE,
  KEY `book_shortage_id` (`book_id`),
  KEY `supplier_book_shortage_id` (`supplier_id`),
  KEY `shortage_id` (`shortage_id`),
  CONSTRAINT `book_shortage_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `shortage_id` FOREIGN KEY (`shortage_id`) REFERENCES `shortages` (`shortage_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `supplier_book_shortage_id` FOREIGN KEY (`supplier_id`) REFERENCES `suppliers` (`supplier_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of book_shortages
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for book_suppliers
-- ----------------------------
DROP TABLE IF EXISTS `book_suppliers`;
CREATE TABLE `book_suppliers` (
  `book_id` int unsigned NOT NULL,
  `supplier_id` int unsigned NOT NULL,
  PRIMARY KEY (`book_id`,`supplier_id`),
  KEY `book_suppliers_id` (`supplier_id`),
  CONSTRAINT `book_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `book_suppliers_id` FOREIGN KEY (`supplier_id`) REFERENCES `suppliers` (`supplier_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of book_suppliers
-- ----------------------------
BEGIN;
INSERT INTO `book_suppliers` (`book_id`, `supplier_id`) VALUES (1, 1);
INSERT INTO `book_suppliers` (`book_id`, `supplier_id`) VALUES (2, 1);
COMMIT;

-- ----------------------------
-- Table structure for books
-- ----------------------------
DROP TABLE IF EXISTS `books`;
CREATE TABLE `books` (
  `book_id` int unsigned NOT NULL AUTO_INCREMENT,
  `isbn` varchar(24) COLLATE utf8mb4_general_ci NOT NULL,
  `title` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `publisher_id` int unsigned NOT NULL,
  `price` decimal(10,2) unsigned NOT NULL DEFAULT '0.00',
  `catalog` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `cover` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `is_onstore` tinyint unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`book_id`,`isbn`) USING BTREE,
  UNIQUE KEY `book_id` (`book_id`) USING BTREE,
  UNIQUE KEY `isbn` (`isbn`) USING BTREE,
  KEY `publisher_id` (`publisher_id`),
  CONSTRAINT `publisher_id` FOREIGN KEY (`publisher_id`) REFERENCES `publishers` (`publisher_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of books
-- ----------------------------
BEGIN;
INSERT INTO `books` (`book_id`, `isbn`, `title`, `publisher_id`, `price`, `catalog`, `cover`, `is_onstore`) VALUES (1, '9789888864690', 'Butterworths Hong Kong Employment Law Handbook 8th ed', 1, 2409.14, '', '/assets/images/9789888864690.png', 0);
INSERT INTO `books` (`book_id`, `isbn`, `title`, `publisher_id`, `price`, `catalog`, `cover`, `is_onstore`) VALUES (2, '9781509972913', 'Chinese Civil Code: Specific Parts', 2, 2398.50, '', '/assets/images/9781509972913.png', 0);
COMMIT;

-- ----------------------------
-- Table structure for credit_rules
-- ----------------------------
DROP TABLE IF EXISTS `credit_rules`;
CREATE TABLE `credit_rules` (
  `credit_level` int NOT NULL,
  `discount_percentage` decimal(3,1) DEFAULT NULL,
  `overdraft_limit` decimal(10,2) DEFAULT NULL,
  `auto_upgrade_balance` decimal(10,2) DEFAULT NULL,
  `auto_upgrade_total_purchase` decimal(10,2) DEFAULT NULL,
  PRIMARY KEY (`credit_level`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of credit_rules
-- ----------------------------
BEGIN;
INSERT INTO `credit_rules` (`credit_level`, `discount_percentage`, `overdraft_limit`, `auto_upgrade_balance`, `auto_upgrade_total_purchase`) VALUES (1, 10.0, 0.00, 200.00, 1000.00);
INSERT INTO `credit_rules` (`credit_level`, `discount_percentage`, `overdraft_limit`, `auto_upgrade_balance`, `auto_upgrade_total_purchase`) VALUES (2, 15.0, 0.00, 500.00, 2500.00);
INSERT INTO `credit_rules` (`credit_level`, `discount_percentage`, `overdraft_limit`, `auto_upgrade_balance`, `auto_upgrade_total_purchase`) VALUES (3, 15.0, 500.00, 1000.00, 5000.00);
INSERT INTO `credit_rules` (`credit_level`, `discount_percentage`, `overdraft_limit`, `auto_upgrade_balance`, `auto_upgrade_total_purchase`) VALUES (4, 20.0, 1000.00, 2000.00, 10000.00);
INSERT INTO `credit_rules` (`credit_level`, `discount_percentage`, `overdraft_limit`, `auto_upgrade_balance`, `auto_upgrade_total_purchase`) VALUES (5, 25.0, 5000.00, 5000.00, 20000.00);
COMMIT;

-- ----------------------------
-- Table structure for customers
-- ----------------------------
DROP TABLE IF EXISTS `customers`;
CREATE TABLE `customers` (
  `customer_id` int unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(50) COLLATE utf8mb4_general_ci NOT NULL,
  `pwd` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT '',
  `email` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `account_balance` decimal(10,2) NOT NULL DEFAULT '0.00',
  `credit_level` tinyint unsigned NOT NULL DEFAULT '1',
  `total_purchase` decimal(10,2) unsigned NOT NULL DEFAULT '0.00',
  `overdraft_limit` decimal(10,2) unsigned NOT NULL DEFAULT '0.00',
  `status` enum('active','cancelled','banned') COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'active',
  PRIMARY KEY (`customer_id`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of customers
-- ----------------------------
BEGIN;
INSERT INTO `customers` (`customer_id`, `username`, `pwd`, `name`, `address`, `email`, `account_balance`, `credit_level`, `total_purchase`, `overdraft_limit`, `status`) VALUES (1, 'root', '6T6Z8vqX', 'root', '', '111@gmail.com', 0.00, 5, 578193.60, 0.00, 'active');
COMMIT;

-- ----------------------------
-- Table structure for keywords
-- ----------------------------
DROP TABLE IF EXISTS `keywords`;
CREATE TABLE `keywords` (
  `keyword_id` int unsigned NOT NULL AUTO_INCREMENT,
  `keyword` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`keyword_id`,`keyword`) USING BTREE,
  KEY `keyword_id` (`keyword_id`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of keywords
-- ----------------------------
BEGIN;
INSERT INTO `keywords` (`keyword_id`, `keyword`) VALUES (1, 'Hong Kong');
INSERT INTO `keywords` (`keyword_id`, `keyword`) VALUES (2, 'Law');
INSERT INTO `keywords` (`keyword_id`, `keyword`) VALUES (3, 'China');
COMMIT;

-- ----------------------------
-- Table structure for loactions
-- ----------------------------
DROP TABLE IF EXISTS `loactions`;
CREATE TABLE `loactions` (
  `location_id` int unsigned NOT NULL AUTO_INCREMENT,
  `description` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`location_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of loactions
-- ----------------------------
BEGIN;
INSERT INTO `loactions` (`location_id`, `description`) VALUES (1, 'Base');
COMMIT;

-- ----------------------------
-- Table structure for order_items
-- ----------------------------
DROP TABLE IF EXISTS `order_items`;
CREATE TABLE `order_items` (
  `order_item_id` int unsigned NOT NULL AUTO_INCREMENT,
  `order_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `quantity` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`order_item_id`,`order_id`,`book_id`) USING BTREE,
  KEY `order_item_order_id` (`order_id`),
  KEY `book_order_item_id` (`book_id`),
  CONSTRAINT `book_order_item_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `order_item_order_id` FOREIGN KEY (`order_id`) REFERENCES `orders` (`order_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of order_items
-- ----------------------------
BEGIN;
INSERT INTO `order_items` (`order_item_id`, `order_id`, `book_id`, `quantity`) VALUES (1, 1, 1, 10);
INSERT INTO `order_items` (`order_item_id`, `order_id`, `book_id`, `quantity`) VALUES (2, 1, 2, 20);
COMMIT;

-- ----------------------------
-- Table structure for orders
-- ----------------------------
DROP TABLE IF EXISTS `orders`;
CREATE TABLE `orders` (
  `order_id` int unsigned NOT NULL AUTO_INCREMENT,
  `customer_id` int unsigned NOT NULL,
  `order_date` datetime NOT NULL,
  `shipping_address` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `payment_status` enum('unpaid','paid') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'unpaid',
  `shipping_status` enum('pending','shipped','partially_shipped','partially_delivered','delivered') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'pending',
  PRIMARY KEY (`order_id`,`customer_id`) USING BTREE,
  KEY `custom_order_id` (`customer_id`),
  KEY `order_id` (`order_id`),
  CONSTRAINT `custom_order_id` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`customer_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of orders
-- ----------------------------
BEGIN;
INSERT INTO `orders` (`order_id`, `customer_id`, `order_date`, `shipping_address`, `payment_status`, `shipping_status`) VALUES (1, 1, '2024-12-12 16:34:00', '', 'unpaid', 'pending');
COMMIT;

-- ----------------------------
-- Table structure for price_inquiries
-- ----------------------------
DROP TABLE IF EXISTS `price_inquiries`;
CREATE TABLE `price_inquiries` (
  `price_inquiry_id` int unsigned NOT NULL AUTO_INCREMENT,
  `custom_id` int unsigned NOT NULL,
  `book_title` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `isbn` varchar(24) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `inquiry_date` datetime NOT NULL,
  `status` enum('pending','quoted','closed') COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'pending',
  PRIMARY KEY (`price_inquiry_id`,`custom_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of price_inquiries
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for publishers
-- ----------------------------
DROP TABLE IF EXISTS `publishers`;
CREATE TABLE `publishers` (
  `publisher_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`publisher_id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of publishers
-- ----------------------------
BEGIN;
INSERT INTO `publishers` (`publisher_id`, `name`) VALUES (1, 'LexisNexis');
INSERT INTO `publishers` (`publisher_id`, `name`) VALUES (2, 'Hart Publishing');
COMMIT;

-- ----------------------------
-- Table structure for purchase_order_items
-- ----------------------------
DROP TABLE IF EXISTS `purchase_order_items`;
CREATE TABLE `purchase_order_items` (
  `order_item_id` int unsigned NOT NULL AUTO_INCREMENT,
  `purchase_order_id` int unsigned NOT NULL,
  `supplier_catalog_id` int unsigned NOT NULL,
  `quantity` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`order_item_id`,`purchase_order_id`,`supplier_catalog_id`) USING BTREE,
  KEY `purchase_order_id` (`purchase_order_id`),
  KEY `order_item_supplier_catalog_id` (`supplier_catalog_id`),
  CONSTRAINT `order_item_supplier_catalog_id` FOREIGN KEY (`supplier_catalog_id`) REFERENCES `supplier_catalogs` (`supplier_catalog_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `purchase_order_id` FOREIGN KEY (`purchase_order_id`) REFERENCES `purchase_orders` (`purchase_order_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of purchase_order_items
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for purchase_orders
-- ----------------------------
DROP TABLE IF EXISTS `purchase_orders`;
CREATE TABLE `purchase_orders` (
  `purchase_order_id` int unsigned NOT NULL AUTO_INCREMENT,
  `order_date` datetime NOT NULL,
  `expected_delivery_date` datetime NOT NULL,
  `status` enum('pending','received','cancelled','partial_received','completed') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'pending',
  PRIMARY KEY (`purchase_order_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of purchase_orders
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for series
-- ----------------------------
DROP TABLE IF EXISTS `series`;
CREATE TABLE `series` (
  `series_id` int unsigned NOT NULL AUTO_INCREMENT,
  `series_title` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`series_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of series
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for series_books
-- ----------------------------
DROP TABLE IF EXISTS `series_books`;
CREATE TABLE `series_books` (
  `series_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `column_num` int unsigned NOT NULL,
  PRIMARY KEY (`series_id`,`book_id`),
  KEY `book_series_id` (`book_id`),
  CONSTRAINT `book_series_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `series_book_id` FOREIGN KEY (`series_id`) REFERENCES `series` (`series_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of series_books
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for shortages
-- ----------------------------
DROP TABLE IF EXISTS `shortages`;
CREATE TABLE `shortages` (
  `shortage_id` int unsigned NOT NULL AUTO_INCREMENT,
  `registration_date` datetime NOT NULL,
  `is_resolved` tinyint unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`shortage_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of shortages
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for supplier_catalogs
-- ----------------------------
DROP TABLE IF EXISTS `supplier_catalogs`;
CREATE TABLE `supplier_catalogs` (
  `supplier_catalog_id` int unsigned NOT NULL AUTO_INCREMENT,
  `supplier_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `available_quantity` int unsigned NOT NULL DEFAULT '0',
  `price` decimal(10,2) unsigned NOT NULL,
  PRIMARY KEY (`supplier_catalog_id`,`supplier_id`,`book_id`) USING BTREE,
  KEY `supplier_catalog_id` (`supplier_catalog_id`),
  KEY `catalog_book_id` (`book_id`),
  CONSTRAINT `catalog_book_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of supplier_catalogs
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for supplier_records
-- ----------------------------
DROP TABLE IF EXISTS `supplier_records`;
CREATE TABLE `supplier_records` (
  `supplier_record_id` int NOT NULL,
  `supplier_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `supply_price` decimal(10,2) NOT NULL,
  `supply_date` datetime NOT NULL,
  `quantity_supplied` int unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`supplier_record_id`,`supplier_id`,`book_id`) USING BTREE,
  KEY `record_supplier_id` (`supplier_id`),
  KEY `record_book_id` (`book_id`),
  CONSTRAINT `record_book_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `record_supplier_id` FOREIGN KEY (`supplier_id`) REFERENCES `suppliers` (`supplier_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of supplier_records
-- ----------------------------
BEGIN;
COMMIT;

-- ----------------------------
-- Table structure for suppliers
-- ----------------------------
DROP TABLE IF EXISTS `suppliers`;
CREATE TABLE `suppliers` (
  `supplier_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `telephone` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `address` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `fax` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`supplier_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;

-- ----------------------------
-- Records of suppliers
-- ----------------------------
BEGIN;
INSERT INTO `suppliers` (`supplier_id`, `name`, `telephone`, `email`, `address`, `fax`) VALUES (1, 'Bloomsbury Books Ltd', '(852) 2526 5387', 'books@bloomsbury.com.hk', 'Room 1304-06, Eastern Centre, 1065 King\'s Road, Quarry Bay, Hong Kong', '(852) 2877 0755');
COMMIT;

-- ----------------------------
-- Event structure for auto_upgrade
-- ----------------------------
DROP EVENT IF EXISTS `auto_upgrade`;
delimiter ;;
CREATE EVENT `auto_upgrade`
ON SCHEDULE
EVERY '1' MONTH STARTS '2024-12-01 00:00:00'
DO BEGIN
UPDATE customers
SET
customers.credit_level = 
CASE 
WHEN customers.total_purchase < 1000 OR customers.account_balance < 200 THEN 1
    WHEN customers.total_purchase >= 50000 OR customers.account_balance >= 5000 THEN 5
    WHEN customers.total_purchase >= 10000 OR customers.account_balance >= 2000 THEN 4
    WHEN customers.total_purchase >= 5000 OR customers.account_balance >= 1000 THEN 3
    WHEN customers.total_purchase >= 2500 OR customers.account_balance >= 500 THEN 2
END;
END
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table book_authors
-- ----------------------------
DROP TRIGGER IF EXISTS `author_limit_trigger`;
delimiter ;;
CREATE TRIGGER `bookstore`.`author_limit_trigger` BEFORE INSERT ON `book_authors` FOR EACH ROW BEGIN
DECLARE author_count INT;

SELECT COUNT(book_authors.author_id) INTO author_count
FROM book_authors
WHERE
book_authors.book_id = NEW.book_id;

IF author_count > 4 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Authors limit exceeded, cannot insert more authors.';
END IF;
END
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table book_authors
-- ----------------------------
DROP TRIGGER IF EXISTS `author_duplicate_trigger`;
delimiter ;;
CREATE TRIGGER `bookstore`.`author_duplicate_trigger` BEFORE INSERT ON `book_authors` FOR EACH ROW BEGIN
DECLARE author_count INT;

SELECT COUNT(book_authors.author_id) INTO author_count
FROM book_authors
WHERE
book_authors.`order` = NEW.`order`;

IF author_count > 2 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Duplicate author in the same order, cannot insert more authors with the same order.';
END IF;
END
;;
delimiter ;

-- ----------------------------
-- Triggers structure for table book_keywords
-- ----------------------------
DROP TRIGGER IF EXISTS `keyword_limit_trigger`;
delimiter ;;
CREATE TRIGGER `bookstore`.`keyword_limit_trigger` BEFORE INSERT ON `book_keywords` FOR EACH ROW BEGIN
DECLARE keyword_count INT;

SELECT COUNT(book_keywords.keyword_id) INTO keyword_count
FROM book_keywords
WHERE
book_keywords.book_id = NEW.book_id;

IF keyword_count > 10 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Keywords limit exceeded, cannot insert more keywords.';
END IF;
END
;;
delimiter ;

SET FOREIGN_KEY_CHECKS = 1;
