-- Your SQL goes here
-- Your SQL goes here
CREATE TABLE IF NOT EXISTS rustaceans (
  id INT AUTO_INCREMENT PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  email VARCHAR(255) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
)  ENGINE=INNODB;