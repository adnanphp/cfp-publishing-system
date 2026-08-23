-- Insert sample data into downloads table
INSERT INTO downloads (member_id, text_id, download_date, ip_address, user_agent, country) VALUES
-- Downloads for member_id 1
(1, 1, '2024-01-15 10:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 2, '2024-01-16 14:20:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 3, '2024-01-17 09:15:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 5, '2024-01-18 16:45:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),

-- Downloads for member_id 2
(2, 1, '2024-01-20 11:10:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 4, '2024-01-21 13:25:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 6, '2024-01-22 15:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),

-- Downloads for member_id 3
(3, 2, '2024-01-25 08:45:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 3, '2024-01-26 10:20:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 7, '2024-01-27 12:00:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),

-- Downloads for member_id 4
(4, 1, '2024-02-01 09:30:00', '203.0.113.5', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(4, 5, '2024-02-02 14:15:00', '203.0.113.5', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(4, 8, '2024-02-03 16:45:00', '203.0.113.5', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),

-- Downloads for member_id 5
(5, 2, '2024-02-05 11:00:00', '198.51.100.10', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),
(5, 4, '2024-02-06 13:20:00', '198.51.100.10', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),
(5, 9, '2024-02-07 15:40:00', '198.51.100.10', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),

-- Multiple downloads for popular texts (text_id 1 and 2)
(6, 1, '2024-02-10 10:00:00', '192.168.1.200', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(7, 1, '2024-02-11 14:30:00', '10.0.0.75', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(8, 2, '2024-02-12 09:15:00', '172.16.0.50', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(9, 2, '2024-02-13 16:00:00', '203.0.113.25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(10, 1, '2024-02-14 11:45:00', '198.51.100.35', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),

-- Downloads for other texts
(6, 3, '2024-02-15 13:20:00', '192.168.1.200', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(7, 4, '2024-02-16 15:10:00', '10.0.0.75', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(8, 5, '2024-02-17 09:45:00', '172.16.0.50', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(9, 6, '2024-02-18 14:30:00', '203.0.113.25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(10, 7, '2024-02-19 16:15:00', '198.51.100.35', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),

-- More recent downloads
(1, 10, '2024-03-01 10:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(2, 10, '2024-03-02 14:20:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(3, 10, '2024-03-03 09:15:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(4, 10, '2024-03-04 16:45:00', '203.0.113.5', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(5, 10, '2024-03-05 11:10:00', '198.51.100.10', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),

-- Downloads from different countries
(11, 1, '2024-03-10 08:00:00', '200.100.50.25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'BR'),
(12, 2, '2024-03-11 12:30:00', '150.200.100.75', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'IN'),
(13, 3, '2024-03-12 15:45:00', '180.250.150.50', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'JP'),
(14, 4, '2024-03-13 10:20:00', '95.100.200.25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'FR'),
(15, 5, '2024-03-14 14:10:00', '85.150.100.75', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'IT'),

-- Same member downloading multiple times
(1, 1, '2024-03-20 09:00:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 1, '2024-03-25 14:00:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(2, 2, '2024-03-21 10:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 2, '2024-03-26 15:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA');
