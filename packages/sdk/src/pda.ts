import { PublicKey } from '@solana/web3.js';
import { PROGRAM_ADDRESS } from './generated';

export const PREFIX: string = 'JSON';

export function find_metadata_account(json_account: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from(PREFIX), new PublicKey(PROGRAM_ADDRESS).toBuffer(), json_account.toBuffer()],
    new PublicKey(PROGRAM_ADDRESS),
  );
}
