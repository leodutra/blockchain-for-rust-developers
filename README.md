# Blockchain for Rust Developers

Suggested implementation for learning the basics of blockchain from the book Blockchain for Rust Developers.

![Blockchain for Rust Developers - cover](./book-cover.jpg "Blockchain for Rust Developers")

## Example Output

```
Block Mined
true
Blockchain {
    blocks: [
        Block {
            timestamp: 1637959701,
            hash: "005086bf6288cbcfbfaf6f407508b321446a5989e55b5c0d6de8e020d2bc3630",
            pre_hash: "0",
            transactions: [
                Transaction {
                    sender: Some(
                        PublicKey(CompressedEdwardsY: [190, 51, 125, 140, 108, 216, 110, 117, 41, 66, 223, 5, 20, 22, 229, 211, 154, 32, 62, 57, 175, 176, 206, 189, 13, 86, 25, 27, 117, 128, 143, 140]), EdwardsPoint{
                                X: FieldElement51([1793994831935305, 246449421626808, 2003592315437596, 1012280055205543, 387755004590414]),
                                Y: FieldElement51([1877302986625477, 554722940722936, 817897772537920, 1127770698425192, 2079932546861303]),
                                Z: FieldElement51([85269939129448, 1264024125449095, 113273163267861, 1059619499414825, 97237585708608]),
                                T: FieldElement51([989628748793107, 1555535653475237, 1194814713001641, 1526053629402434, 1970596239685738])
                        }),
                    ),
                    receiver: Some(
                        PublicKey(CompressedEdwardsY: [199, 214, 70, 8, 240, 238, 174, 81, 252, 119, 70, 81, 59, 113, 195, 70, 162, 113, 57, 212, 42, 76, 183, 74, 159, 170, 72, 158, 57, 229, 186, 152]), EdwardsPoint{
                                X: FieldElement51([2021131103139391, 1554570240062283, 1017412822303322, 2101231224914854, 892009561364285]),
                                Y: FieldElement51([1756590567669725, 1362605289168873, 596311249728424, 728722157411811, 470853847402495]),
                                Z: FieldElement51([1378539204457598, 2251107728738043, 1613566969863317, 959398320767143, 60228371763522]),
                                T: FieldElement51([25813870210486, 1432491347943525, 1296264533914264, 345636757110934, 828254290974560])
                        }),
                    ),
                    amount: 2000.0,
                    signature: Some(
                        ed25519::Signature(DE065FE41B0E4C3D4D24D5CED7D970D39FA11C3F711BDF9DAD7F06AD6E286390C983889AFB3E56F132A222EA5ACE550AFAF32AFCABE205A13558F68C25E10C0A),
                    ),
                },
                Transaction {
                    sender: None,
                    receiver: Some(
                        PublicKey(CompressedEdwardsY: [77, 135, 228, 107, 13, 134, 39, 145, 103, 231, 189, 50, 117, 158, 23, 12, 44, 147, 189, 122, 66, 225, 141, 208, 194, 186, 121, 177, 79, 249, 175, 148]), EdwardsPoint{
                                X: FieldElement51([1024253934854838, 1735650274078426, 1348693763491069, 473459837156311, 1179057098295092]),
                                Y: FieldElement51([968238742741363, 753343442449114, 161474355682495, 742196004592982, 1010060009980043]),
                                Z: FieldElement51([1883968189188069, 1219381031504190, 379971888077508, 1603581408739764, 2198145711215675]),
                                T: FieldElement51([321717175998572, 135774110832851, 1862353319275548, 1538683735563946, 1673288022089529])
                        }),
                    ),
                    amount: 100.0,
                    signature: None,
                },
            ],
            nonce: 336,
        },
    ],
    unmined_transactions: [],
    mining_reward: 100.0,
}
```

## License

This code is part of the book Blockchain for Rust Developers (Ayush Kumar Mishra) and should not be used or modified without express authorization from the author.
