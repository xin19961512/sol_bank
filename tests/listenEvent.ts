import * as anchor from "@coral-xyz/anchor";
import { BorshCoder, EventParser, Program } from "@coral-xyz/anchor";
import { SolanaBridge } from "../target/types/solana_bridge";

describe("emit", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.SolanaBridge as Program<SolanaBridge>;
  
    it("listen event!", async () => {
        const EventNative = program.addEventListener('DepositNative', (event, slot) => {
            console.log(`DepositNative from: ${event.from}, to: ${event.to}, chain: ${event.chain}, amout: ${event.value}, targetAddr: ${event.addr}`);
        });

        const EventFt = program.addEventListener('DepositFt', (event, slot) => {
            console.log(`DepositFt from: ${event.from}, to: ${event.to},mint_account:${event.mint}, chain: ${event.chain}, amout: ${event.value}, targetAddr: ${event.addr}`);
        });

        // This line is only for test purposes to ensure the event
        // listener has time to listen to event.

        await new Promise((resolve) => setTimeout(resolve, 10000));

        program.removeEventListener(EventNative);
        program.removeEventListener(EventFt);
    });
});