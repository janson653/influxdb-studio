#!/bin/bash

# 等待InfluxDB启动
echo "等待InfluxDB启动..."
sleep 10

# 创建测试数据库
echo "创建测试数据库..."
influx -execute "CREATE DATABASE testdb"

# 创建测试测量值
echo "创建测试测量值..."
influx -database testdb -execute "
INSERT cpu_usage,host=server01,region=us-west value=0.64 1640995200000000000
INSERT cpu_usage,host=server01,region=us-west value=0.65 1640995260000000000
INSERT cpu_usage,host=server01,region=us-west value=0.66 1640995320000000000
INSERT cpu_usage,host=server02,region=us-east value=0.45 1640995200000000000
INSERT cpu_usage,host=server02,region=us-east value=0.46 1640995260000000000
INSERT cpu_usage,host=server02,region=us-east value=0.47 1640995320000000000
"

# 创建内存使用测量值
echo "创建内存使用测量值..."
influx -database testdb -execute "
INSERT memory_usage,host=server01,region=us-west value=85.2 1640995200000000000
INSERT memory_usage,host=server01,region=us-west value=85.5 1640995260000000000
INSERT memory_usage,host=server01,region=us-west value=85.8 1640995320000000000
INSERT memory_usage,host=server02,region=us-east value=72.1 1640995200000000000
INSERT memory_usage,host=server02,region=us-east value=72.3 1640995260000000000
INSERT memory_usage,host=server02,region=us-east value=72.5 1640995320000000000
"

# 创建网络流量测量值
echo "创建网络流量测量值..."
influx -database testdb -execute "
INSERT network_traffic,host=server01,interface=eth0 bytes_in=1024,bytes_out=2048 1640995200000000000
INSERT network_traffic,host=server01,interface=eth0 bytes_in=1536,bytes_out=3072 1640995260000000000
INSERT network_traffic,host=server01,interface=eth0 bytes_in=2048,bytes_out=4096 1640995320000000000
INSERT network_traffic,host=server02,interface=eth0 bytes_in=512,bytes_out=1024 1640995200000000000
INSERT network_traffic,host=server02,interface=eth0 bytes_in=768,bytes_out=1536 1640995260000000000
INSERT network_traffic,host=server02,interface=eth0 bytes_in=1024,bytes_out=2048 1640995320000000000
"

# 创建温度传感器测量值
echo "创建温度传感器测量值..."
influx -database testdb -execute "
INSERT temperature,sensor=temp01,location=room1 value=22.5 1640995200000000000
INSERT temperature,sensor=temp01,location=room1 value=22.6 1640995260000000000
INSERT temperature,sensor=temp01,location=room1 value=22.7 1640995320000000000
INSERT temperature,sensor=temp02,location=room2 value=23.1 1640995200000000000
INSERT temperature,sensor=temp02,location=room2 value=23.2 1640995260000000000
INSERT temperature,sensor=temp02,location=room2 value=23.3 1640995320000000000
"

echo "InfluxDB初始化完成！"
echo "测试数据库: testdb"
echo "测试测量值: cpu_usage, memory_usage, network_traffic, temperature" 