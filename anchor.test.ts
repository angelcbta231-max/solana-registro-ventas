import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

describe("registro_ventas", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.RegistroVentas as Program<any>;

  const negocio = anchor.web3.Keypair.generate();
  const cliente = anchor.web3.Keypair.generate();
  const cliente2 = anchor.web3.Keypair.generate();
  const venta = anchor.web3.Keypair.generate();
  const venta2 = anchor.web3.Keypair.generate();

  it("1. Crear Registro y Cliente", async () => {
    await program.methods.inicializarNegocio()
      .accounts({ negocio: negocio.publicKey, autoridad: provider.wallet.publicKey })
      .signers([negocio]).rpc();
    await program.methods.crearCliente("Angel")
      .accounts({ negocio: negocio.publicKey, cliente: cliente.publicKey, dueno: provider.wallet.publicKey })
      .signers([cliente]).rpc();
  });

  it("2. Registrar venta valida", async () => {
    await program.methods.crearVenta(new anchor.BN(100))
      .accounts({ negocio: negocio.publicKey, cliente: cliente.publicKey, venta: venta.publicKey, autoridad: provider.wallet.publicKey })
      .signers([venta]).rpc();
  });

  it("3. Corregir importe", async () => {
    await program.methods.corregirImporte(new anchor.BN(150))
      .accounts({ cliente: cliente.publicKey, venta: venta.publicKey, dueno: provider.wallet.publicKey })
      .rpc();
  });

  it("4. Cerrar venta / Devolucion", async () => {
    await program.methods.cerrarVenta()
      .accounts({ cliente: cliente.publicKey, venta: venta.publicKey, dueno: provider.wallet.publicKey })
      .rpc();
  });

  it("5. Dar de baja cliente vacio", async () => {
    await program.methods.darDeBajaCliente()
      .accounts({ cliente: cliente.publicKey, dueno: provider.wallet.publicKey })
      .rpc();
  });

  it("6. Wallet no autorizada DEBE FALLAR", async () => {
    const otro = anchor.web3.Keypair.generate();
    try {
      await program.methods.corregirImporte(new anchor.BN(999))
        .accounts({ cliente: cliente2.publicKey, venta: venta.publicKey, dueno: otro.publicKey })
        .signers([otro]).rpc();
      throw new Error("Debio fallar");
    } catch (e: any) {
      if (e.message === "Debio fallar") throw e;
    }
  });

  it("7. Venta ajena DEBE FALLAR", async () => {
    await program.methods.crearCliente("Otro")
      .accounts({ negocio: negocio.publicKey, cliente: cliente2.publicKey, dueno: provider.wallet.publicKey })
      .signers([cliente2]).rpc();
    await program.methods.crearVenta(new anchor.BN(50))
      .accounts({ negocio: negocio.publicKey, cliente: cliente2.publicKey, venta: venta2.publicKey, autoridad: provider.wallet.publicKey })
      .signers([venta2]).rpc();
    try {
      await program.methods.corregirImporte(new anchor.BN(999))
        .accounts({ cliente: cliente.publicKey, venta: venta2.publicKey, dueno: provider.wallet.publicKey })
        .rpc();
      throw new Error("Debio fallar");
    } catch (e: any) {
      if (e.message === "Debio fallar") throw e;
    }
  });

  it("8. Dar de baja con ventas activas DEBE FALLAR", async () => {
    try {
      await program.methods.darDeBajaCliente()
        .accounts({ cliente: cliente2.publicKey, dueno: provider.wallet.publicKey })
        .rpc();
      throw new Error("Debio fallar");
    } catch (e: any) {
      if (e.message === "Debio fallar") throw e;
    }
  });
});