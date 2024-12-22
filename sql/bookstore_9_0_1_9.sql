-- MySQL dump 10.13  Distrib 9.0.1, for macos15.1 (arm64)
--
-- Host: 127.0.0.1    Database: bookstore
-- ------------------------------------------------------
-- Server version	8.0.40

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8mb4 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `admins`
--

DROP TABLE IF EXISTS `admins`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `admins` (
  `admin_id` int unsigned NOT NULL AUTO_INCREMENT,
  `admin_username` varchar(50) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `admin_pwd` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `status` enum('active','cancelled') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'active',
  `role` enum('admin','staff') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'staff',
  PRIMARY KEY (`admin_id`),
  UNIQUE KEY `admin_username` (`admin_username`) USING BTREE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `admins`
--

LOCK TABLES `admins` WRITE;
/*!40000 ALTER TABLE `admins` DISABLE KEYS */;
INSERT INTO `admins` VALUES (1,'root','tN88Bj4=','active','admin'),(2,'reinerina','45h9W2VXv8K8bg==','active','admin');
/*!40000 ALTER TABLE `admins` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `authed_customers`
--

DROP TABLE IF EXISTS `authed_customers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `authed_customers` (
  `customer_id` int unsigned NOT NULL,
  `token` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `last_used` datetime NOT NULL,
  `is_online` tinyint unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`customer_id`),
  CONSTRAINT `authed_customer_id` FOREIGN KEY (`customer_id`) REFERENCES `customers` (`customer_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `authed_customers`
--

LOCK TABLES `authed_customers` WRITE;
/*!40000 ALTER TABLE `authed_customers` DISABLE KEYS */;
INSERT INTO `authed_customers` VALUES (1,'B5P2uHvQi94W7QZsHMBEIkNCyJN4x3x+SZqkCoARN+fV481xe7Bjf9WDOltk','2024-12-19 19:17:06',1);
/*!40000 ALTER TABLE `authed_customers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `authors`
--

DROP TABLE IF EXISTS `authors`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `authors` (
  `author_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`author_id`),
  FULLTEXT KEY `name` (`name`) /*!50100 WITH PARSER `ngram` */ 
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `authors`
--

LOCK TABLES `authors` WRITE;
/*!40000 ALTER TABLE `authors` DISABLE KEYS */;
INSERT INTO `authors` VALUES (1,'Michael Downey'),(2,'Patricia Yeung'),(3,'Yuanshi Bu');
/*!40000 ALTER TABLE `authors` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `book_authors`
--

DROP TABLE IF EXISTS `book_authors`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `book_authors` (
  `book_id` int unsigned NOT NULL,
  `author_id` int unsigned NOT NULL,
  `order` tinyint NOT NULL,
  PRIMARY KEY (`book_id`,`author_id`),
  KEY `author_id` (`author_id`),
  CONSTRAINT `author_id` FOREIGN KEY (`author_id`) REFERENCES `authors` (`author_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `book_author_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `book_authors`
--

LOCK TABLES `book_authors` WRITE;
/*!40000 ALTER TABLE `book_authors` DISABLE KEYS */;
INSERT INTO `book_authors` VALUES (1,1,1),(1,2,2),(2,3,1);
/*!40000 ALTER TABLE `book_authors` ENABLE KEYS */;
UNLOCK TABLES;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_0900_ai_ci */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION' */ ;
DELIMITER ;;
/*!50003 CREATE*/ /*!50017 DEFINER=`reinerina`@`localhost`*/ /*!50003 TRIGGER `author_limit_trigger` BEFORE INSERT ON `book_authors` FOR EACH ROW BEGIN
DECLARE author_count INT;

SELECT COUNT(book_authors.author_id) INTO author_count
FROM book_authors
WHERE
book_authors.book_id = NEW.book_id;

IF author_count > 4 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Authors limit exceeded, cannot insert more authors.';
END IF;
END */;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_0900_ai_ci */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION' */ ;
DELIMITER ;;
/*!50003 CREATE*/ /*!50017 DEFINER=`reinerina`@`localhost`*/ /*!50003 TRIGGER `author_duplicate_trigger` BEFORE INSERT ON `book_authors` FOR EACH ROW BEGIN
DECLARE author_count INT;

SELECT COUNT(book_authors.author_id) INTO author_count
FROM book_authors
WHERE
book_authors.`order` = NEW.`order`;

IF author_count > 2 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Duplicate author in the same order, cannot insert more authors with the same order.';
END IF;
END */;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;

--
-- Table structure for table `book_keywords`
--

DROP TABLE IF EXISTS `book_keywords`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `book_keywords` (
  `book_id` int unsigned NOT NULL,
  `keyword_id` int unsigned NOT NULL,
  PRIMARY KEY (`book_id`,`keyword_id`) USING BTREE,
  KEY `keyword_id` (`keyword_id`),
  CONSTRAINT `keyword_id` FOREIGN KEY (`keyword_id`) REFERENCES `keywords` (`keyword_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `book_keywords`
--

LOCK TABLES `book_keywords` WRITE;
/*!40000 ALTER TABLE `book_keywords` DISABLE KEYS */;
INSERT INTO `book_keywords` VALUES (1,1),(1,2),(2,2),(2,3);
/*!40000 ALTER TABLE `book_keywords` ENABLE KEYS */;
UNLOCK TABLES;
/*!50003 SET @saved_cs_client      = @@character_set_client */ ;
/*!50003 SET @saved_cs_results     = @@character_set_results */ ;
/*!50003 SET @saved_col_connection = @@collation_connection */ ;
/*!50003 SET character_set_client  = utf8mb4 */ ;
/*!50003 SET character_set_results = utf8mb4 */ ;
/*!50003 SET collation_connection  = utf8mb4_0900_ai_ci */ ;
/*!50003 SET @saved_sql_mode       = @@sql_mode */ ;
/*!50003 SET sql_mode              = 'ONLY_FULL_GROUP_BY,STRICT_TRANS_TABLES,NO_ZERO_IN_DATE,NO_ZERO_DATE,ERROR_FOR_DIVISION_BY_ZERO,NO_ENGINE_SUBSTITUTION' */ ;
DELIMITER ;;
/*!50003 CREATE*/ /*!50017 DEFINER=`reinerina`@`localhost`*/ /*!50003 TRIGGER `keyword_limit_trigger` BEFORE INSERT ON `book_keywords` FOR EACH ROW BEGIN
DECLARE keyword_count INT;

SELECT COUNT(book_keywords.keyword_id) INTO keyword_count
FROM book_keywords
WHERE
book_keywords.book_id = NEW.book_id;

IF keyword_count > 10 THEN
SIGNAL SQLSTATE '45000'
SET MESSAGE_TEXT = 'Keywords limit exceeded, cannot insert more keywords.';
END IF;
END */;;
DELIMITER ;
/*!50003 SET sql_mode              = @saved_sql_mode */ ;
/*!50003 SET character_set_client  = @saved_cs_client */ ;
/*!50003 SET character_set_results = @saved_cs_results */ ;
/*!50003 SET collation_connection  = @saved_col_connection */ ;

--
-- Table structure for table `book_loactions`
--

DROP TABLE IF EXISTS `book_loactions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `book_loactions`
--

LOCK TABLES `book_loactions` WRITE;
/*!40000 ALTER TABLE `book_loactions` DISABLE KEYS */;
INSERT INTO `book_loactions` VALUES (1,1,190);
/*!40000 ALTER TABLE `book_loactions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `book_shortages`
--

DROP TABLE IF EXISTS `book_shortages`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `book_shortages`
--

LOCK TABLES `book_shortages` WRITE;
/*!40000 ALTER TABLE `book_shortages` DISABLE KEYS */;
/*!40000 ALTER TABLE `book_shortages` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `book_suppliers`
--

DROP TABLE IF EXISTS `book_suppliers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `book_suppliers` (
  `book_id` int unsigned NOT NULL,
  `supplier_id` int unsigned NOT NULL,
  PRIMARY KEY (`book_id`,`supplier_id`),
  KEY `book_suppliers_id` (`supplier_id`),
  CONSTRAINT `book_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `book_suppliers_id` FOREIGN KEY (`supplier_id`) REFERENCES `suppliers` (`supplier_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `book_suppliers`
--

LOCK TABLES `book_suppliers` WRITE;
/*!40000 ALTER TABLE `book_suppliers` DISABLE KEYS */;
INSERT INTO `book_suppliers` VALUES (1,1),(2,1);
/*!40000 ALTER TABLE `book_suppliers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `books`
--

DROP TABLE IF EXISTS `books`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
  FULLTEXT KEY `title` (`title`) /*!50100 WITH PARSER `ngram` */ ,
  CONSTRAINT `publisher_id` FOREIGN KEY (`publisher_id`) REFERENCES `publishers` (`publisher_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `books`
--

LOCK TABLES `books` WRITE;
/*!40000 ALTER TABLE `books` DISABLE KEYS */;
INSERT INTO `books` VALUES (1,'9789888864690','Butterworths Hong Kong Employment Law Handbook 8th ed',1,2409.14,'','/assets/images/9789888864690.png',0),(2,'9781509972913','Chinese Civil Code: Specific Parts',2,2398.50,'','/assets/images/9781509972913.png',0);
/*!40000 ALTER TABLE `books` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `credit_rules`
--

DROP TABLE IF EXISTS `credit_rules`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `credit_rules` (
  `credit_level` int NOT NULL,
  `discount_percentage` decimal(3,1) DEFAULT NULL,
  `overdraft_limit` decimal(10,2) DEFAULT NULL,
  `auto_upgrade_balance` decimal(10,2) DEFAULT NULL,
  `auto_upgrade_total_purchase` decimal(10,2) DEFAULT NULL,
  PRIMARY KEY (`credit_level`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `credit_rules`
--

LOCK TABLES `credit_rules` WRITE;
/*!40000 ALTER TABLE `credit_rules` DISABLE KEYS */;
INSERT INTO `credit_rules` VALUES (1,10.0,0.00,200.00,1000.00),(2,15.0,0.00,500.00,2500.00),(3,15.0,500.00,1000.00,5000.00),(4,20.0,1000.00,2000.00,10000.00),(5,25.0,5000.00,5000.00,20000.00);
/*!40000 ALTER TABLE `credit_rules` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `customers`
--

DROP TABLE IF EXISTS `customers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `customers`
--

LOCK TABLES `customers` WRITE;
/*!40000 ALTER TABLE `customers` DISABLE KEYS */;
INSERT INTO `customers` VALUES (1,'root','6T6Z8vqX','root','','111@gmail.com',0.00,5,0.00,0.00,'active');
/*!40000 ALTER TABLE `customers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `keywords`
--

DROP TABLE IF EXISTS `keywords`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `keywords` (
  `keyword_id` int unsigned NOT NULL AUTO_INCREMENT,
  `keyword` varchar(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`keyword_id`,`keyword`) USING BTREE,
  KEY `keyword_id` (`keyword_id`),
  FULLTEXT KEY `keyword` (`keyword`) /*!50100 WITH PARSER `ngram` */ 
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `keywords`
--

LOCK TABLES `keywords` WRITE;
/*!40000 ALTER TABLE `keywords` DISABLE KEYS */;
INSERT INTO `keywords` VALUES (1,'Hong Kong'),(2,'Law'),(3,'China');
/*!40000 ALTER TABLE `keywords` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `loactions`
--

DROP TABLE IF EXISTS `loactions`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `loactions` (
  `location_id` int unsigned NOT NULL AUTO_INCREMENT,
  `description` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`location_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `loactions`
--

LOCK TABLES `loactions` WRITE;
/*!40000 ALTER TABLE `loactions` DISABLE KEYS */;
INSERT INTO `loactions` VALUES (1,'Base');
/*!40000 ALTER TABLE `loactions` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `order_items`
--

DROP TABLE IF EXISTS `order_items`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
) ENGINE=InnoDB AUTO_INCREMENT=11 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `order_items`
--

LOCK TABLES `order_items` WRITE;
/*!40000 ALTER TABLE `order_items` DISABLE KEYS */;
INSERT INTO `order_items` VALUES (1,1,1,10),(2,1,2,20),(3,2,1,10),(4,2,2,20),(5,3,1,10),(6,3,2,20),(7,4,2,5),(8,4,1,20),(9,5,1,4),(10,5,2,16);
/*!40000 ALTER TABLE `order_items` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `orders`
--

DROP TABLE IF EXISTS `orders`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `orders`
--

LOCK TABLES `orders` WRITE;
/*!40000 ALTER TABLE `orders` DISABLE KEYS */;
INSERT INTO `orders` VALUES (1,1,'2024-12-12 16:34:00','','unpaid','pending'),(2,1,'2024-12-17 20:24:09','','unpaid','pending'),(3,1,'2024-12-17 20:46:49','','unpaid','pending'),(4,1,'2024-12-17 20:57:03','','unpaid','pending'),(5,1,'2024-12-17 20:58:10','','unpaid','pending');
/*!40000 ALTER TABLE `orders` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `price_inquiries`
--

DROP TABLE IF EXISTS `price_inquiries`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `price_inquiries` (
  `price_inquiry_id` int unsigned NOT NULL AUTO_INCREMENT,
  `custom_id` int unsigned NOT NULL,
  `book_title` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `isbn` varchar(24) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `inquiry_date` datetime NOT NULL,
  `status` enum('pending','quoted','closed') COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'pending',
  PRIMARY KEY (`price_inquiry_id`,`custom_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `price_inquiries`
--

LOCK TABLES `price_inquiries` WRITE;
/*!40000 ALTER TABLE `price_inquiries` DISABLE KEYS */;
/*!40000 ALTER TABLE `price_inquiries` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `publishers`
--

DROP TABLE IF EXISTS `publishers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `publishers` (
  `publisher_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`publisher_id`)
) ENGINE=InnoDB AUTO_INCREMENT=3 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `publishers`
--

LOCK TABLES `publishers` WRITE;
/*!40000 ALTER TABLE `publishers` DISABLE KEYS */;
INSERT INTO `publishers` VALUES (1,'LexisNexis'),(2,'Hart Publishing');
/*!40000 ALTER TABLE `publishers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `purchase_order_items`
--

DROP TABLE IF EXISTS `purchase_order_items`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `purchase_order_items`
--

LOCK TABLES `purchase_order_items` WRITE;
/*!40000 ALTER TABLE `purchase_order_items` DISABLE KEYS */;
/*!40000 ALTER TABLE `purchase_order_items` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `purchase_orders`
--

DROP TABLE IF EXISTS `purchase_orders`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `purchase_orders` (
  `purchase_order_id` int unsigned NOT NULL AUTO_INCREMENT,
  `order_date` datetime NOT NULL,
  `expected_delivery_date` datetime NOT NULL,
  `status` enum('pending','received','cancelled','partial_received','completed') CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL DEFAULT 'pending',
  PRIMARY KEY (`purchase_order_id`) USING BTREE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `purchase_orders`
--

LOCK TABLES `purchase_orders` WRITE;
/*!40000 ALTER TABLE `purchase_orders` DISABLE KEYS */;
/*!40000 ALTER TABLE `purchase_orders` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `series`
--

DROP TABLE IF EXISTS `series`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `series` (
  `series_id` int unsigned NOT NULL AUTO_INCREMENT,
  `series_title` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`series_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `series`
--

LOCK TABLES `series` WRITE;
/*!40000 ALTER TABLE `series` DISABLE KEYS */;
/*!40000 ALTER TABLE `series` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `series_books`
--

DROP TABLE IF EXISTS `series_books`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `series_books` (
  `series_id` int unsigned NOT NULL,
  `book_id` int unsigned NOT NULL,
  `column_num` int unsigned NOT NULL,
  PRIMARY KEY (`series_id`,`book_id`),
  KEY `book_series_id` (`book_id`),
  CONSTRAINT `book_series_id` FOREIGN KEY (`book_id`) REFERENCES `books` (`book_id`) ON DELETE CASCADE ON UPDATE CASCADE,
  CONSTRAINT `series_book_id` FOREIGN KEY (`series_id`) REFERENCES `series` (`series_id`) ON DELETE CASCADE ON UPDATE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `series_books`
--

LOCK TABLES `series_books` WRITE;
/*!40000 ALTER TABLE `series_books` DISABLE KEYS */;
/*!40000 ALTER TABLE `series_books` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `shortages`
--

DROP TABLE IF EXISTS `shortages`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `shortages` (
  `shortage_id` int unsigned NOT NULL AUTO_INCREMENT,
  `registration_date` datetime NOT NULL,
  `is_resolved` tinyint unsigned NOT NULL DEFAULT '0',
  PRIMARY KEY (`shortage_id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `shortages`
--

LOCK TABLES `shortages` WRITE;
/*!40000 ALTER TABLE `shortages` DISABLE KEYS */;
/*!40000 ALTER TABLE `shortages` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `supplier_catalogs`
--

DROP TABLE IF EXISTS `supplier_catalogs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `supplier_catalogs`
--

LOCK TABLES `supplier_catalogs` WRITE;
/*!40000 ALTER TABLE `supplier_catalogs` DISABLE KEYS */;
/*!40000 ALTER TABLE `supplier_catalogs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `supplier_records`
--

DROP TABLE IF EXISTS `supplier_records`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
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
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `supplier_records`
--

LOCK TABLES `supplier_records` WRITE;
/*!40000 ALTER TABLE `supplier_records` DISABLE KEYS */;
/*!40000 ALTER TABLE `supplier_records` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `suppliers`
--

DROP TABLE IF EXISTS `suppliers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `suppliers` (
  `supplier_id` int unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `telephone` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  `email` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `address` varchar(255) COLLATE utf8mb4_general_ci NOT NULL,
  `fax` varchar(255) CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci NOT NULL,
  PRIMARY KEY (`supplier_id`)
) ENGINE=InnoDB AUTO_INCREMENT=2 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_general_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `suppliers`
--

LOCK TABLES `suppliers` WRITE;
/*!40000 ALTER TABLE `suppliers` DISABLE KEYS */;
INSERT INTO `suppliers` VALUES (1,'Bloomsbury Books Ltd','(852) 2526 5387','books@bloomsbury.com.hk','Room 1304-06, Eastern Centre, 1065 King\'s Road, Quarry Bay, Hong Kong','(852) 2877 0755');
/*!40000 ALTER TABLE `suppliers` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-12-22 16:04:56
