anchor test --skip-local-validator

HDXAyUNpESzg8EkJPojmEeNELTQS6BcVmpZ3pR5DyAfo // main account

2nhbqSk4RvZE7jsq3UZmigtmhSVSVDqrWuWN4ViStVZQ // wallet1
[53,159,16,48,248,66,178,138,207,177,15,211,2,64,210,173,44,171,240,79,8,34,106,95,111,70,117,123,114,175,134,76,26,144,187,7,209,146,26,162,50,160,245,91,79,7,209,176,26,102,155,115,82,99,104,215,12,51,79,30,120,223,231,239]

DYPjft89CsTpVVaeUW6CX3aBDh7TifjNJ99XavKmK59j // wallet2
[5,26,140,241,165,51,126,13,95,23,63,154,177,193,224,159,140,128,247,158,117,137,23,201,36,111,174,253,71,155,10,235,186,86,244,157,174,30,247,11,141,80,178,199,172,139,10,11,187,13,131,152,37,230,5,37,57,190,20,41,54,161,61,26]

2UurTVpar9Npe4b1FXNgcVGmowBT3DWJxRX5LSMdjNYq // wallet3
[147,175,13,56,86,2,202,66,198,222,88,2,142,55,202,154,164,187,91,240,96,254,45,147,123,50,15,104,235,13,159,222,22,2,33,39,221,59,185,47,28,193,192,190,122,62,40,114,136,121,117,148,142,248,120,222,130,74,76,152,23,96,8,58]

cargo update -p solana-program@1.18.14 --precise 1.17.28
npm install @solana/spl-token

anchor upgrade --program-id HQW9FafmgcTLLQTjtMaET7ViNiSe5Bk2fEW5jetNivCv target/deploy/solana_bridge.so 

export ALL_PROXY=socks5://127.0.0.1:7890
export ALL_PROXY=socks5://172.16.100.85:7890
export ALL_PROXY="http://192.168.1.34:1080"
anchor deploy --program-name solana_bridge --provider.cluster https://api.mainnet-beta.solana.com
anchor deploy --program-name solana_bridge --provider.cluster https://api.testnet.solana.com 
anchor deploy --provider.cluster https://api.devnet.solana.com
anchor deploy --program-name solana_bridge --provider.cluster http://localhost:8899

3ECu7aijKa79wdaWCLE5oPvTaNmGEe7d1xV26jeiFKyw // playground program
