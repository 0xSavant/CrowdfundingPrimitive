import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { MobiusPrograms } from '../target/types/mobius_programs';

describe('MobiusPrograms', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.MobiusPrograms as Program<MobiusPrograms>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
