-- Populate downloads table for existing members (1, 2, 3) and texts (1-8)
INSERT INTO downloads (member_id, text_id, download_date, ip_address, user_agent, country) VALUES
-- Member 1: John Doe's downloads
(1, 1, '2024-01-15 10:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 2, '2024-01-16 14:20:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 3, '2024-01-17 09:15:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 4, '2024-01-18 16:45:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 5, '2024-01-25 11:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 1, '2024-02-01 09:30:00', '203.0.113.5', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:91.0) Gecko/20100101 Firefox/91.0', 'AU'),
(1, 2, '2024-02-10 14:00:00', '192.168.1.200', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 3, '2024-02-15 16:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),

-- Member 2: Jane Smith's downloads
(2, 1, '2024-01-20 11:10:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 2, '2024-01-21 13:25:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 5, '2024-01-22 15:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 6, '2024-01-23 10:45:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 7, '2024-01-24 14:20:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 1, '2024-02-05 11:00:00', '198.51.100.10', 'Mozilla/5.0 (iPhone; CPU iPhone OS 14_0 like Mac OS X) AppleWebKit/605.1.15', 'DE'),
(2, 2, '2024-02-11 14:30:00', '10.0.0.75', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 3, '2024-02-20 09:15:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),

-- Member 3: Bob Wilson's downloads
(3, 2, '2024-01-25 08:45:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 3, '2024-01-26 10:20:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 4, '2024-01-27 12:00:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 6, '2024-01-28 15:30:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 7, '2024-01-29 09:45:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 8, '2024-01-30 14:15:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 1, '2024-02-12 09:15:00', '172.16.0.50', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 2, '2024-02-18 16:00:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),

-- Recent downloads (March 2024)
(1, 1, '2024-03-01 10:30:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(2, 2, '2024-03-02 14:20:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(3, 1, '2024-03-03 09:15:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(1, 4, '2024-03-10 08:00:00', '200.100.50.25', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'BR'),
(2, 5, '2024-03-11 12:30:00', '150.200.100.75', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'IN'),
(3, 6, '2024-03-12 15:45:00', '180.250.150.50', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'JP'),

-- Multiple downloads of popular texts
(1, 1, '2024-03-20 09:00:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(1, 1, '2024-03-25 14:00:00', '192.168.1.100', 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36', 'US'),
(2, 2, '2024-03-21 10:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(2, 2, '2024-03-26 15:30:00', '10.0.0.50', 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15', 'CA'),
(3, 1, '2024-03-22 11:45:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK'),
(3, 2, '2024-03-27 13:20:00', '172.16.0.25', 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36', 'UK');
