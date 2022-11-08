## Anchor VRF Parser
Template contract to generate vrf client, request randomness and parse random result

`cd anchor-vrf`
> programs/anchor-vrf/src
- Source files for smart contract program

The main idea is that developer can request randomness via CPI from any contract.

Previously vrf client must be generated from client side (front-end).

After that all previously generated data for vrf and vrf client account must be passed in instruction where the CPI will be actuated.

### Build with `anchor build`

### Deploy with `anchor deploy`

### Test with `anchor test`

> NOTE: It`s better to get rid of the "consume_randomness" in production if you use this contract via CPI