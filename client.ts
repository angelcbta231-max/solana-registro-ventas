// Cliente para el boton Run
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);
const program = anchor.workspace.RegistroVentas as Program<any>;

async function main() {
  const negocio = anchor.web3.Keypair.generate();
  const cliente = anchor.web3.Keypair.generate();
  const venta = anchor.web3.Keypair.generate();

  console.log("1. Creando negocio...");
  await program.methods.inicializarNegocio()
    .accounts({ negocio: negocio.publicKey, autoridad: provider.wallet.publicKey })
    .signers([negocio]).rpc();

  console.log("2. Creando cliente Angel...");
  await program.methods.crearCliente("Angel")
    .accounts({ negocio: negocio.publicKey, cliente: cliente.publicKey, dueno: provider.wallet.publicKey })
    .signers([cliente]).rpc();

  console.log("3. Creando venta de 100...");
  await program.methods.crearVenta(new anchor.BN(100))
    .accounts({ negocio: negocio.publicKey, cliente: cliente.publicKey, venta: venta.publicKey, autoridad: provider.wallet.publicKey })
    .signers([venta]).rpc();

  console.log("4. Corrigiendo importe a 150...");
  await program.methods.corregirImporte(new anchor.BN(150))
    .accounts({ cliente: cliente.publicKey, venta: venta.publicKey, dueno: provider.wallet.publicKey })
    .rpc();

  console.log("LISTO! Todo funciono");
  console.log("Negocio:", negocio.publicKey.toString());
  console.log("Cliente:", cliente.publicKey.toString());
  console.log("Venta:", venta.publicKey.toString());
}

main();