use anchor_lang::prelude::*;

declare_id!("11111111111111111111111111111111");

#[program]
pub mod registro_ventas {
    use super::*;

    // 1. Inicializar negocio
    pub fn inicializar_negocio(ctx: Context<InicializarNegocio>) -> Result<()> {
        let negocio = &mut ctx.accounts.negocio;
        negocio.autoridad = ctx.accounts.autoridad.key();
        negocio.contador_ventas = 0;
        negocio.total_clientes = 0;
        Ok(())
    }

    // 1. Crear Cliente
    pub fn crear_cliente(ctx: Context<CrearCliente>, nombre: String) -> Result<()> {
        let negocio = &mut ctx.accounts.negocio;
        let cliente = &mut ctx.accounts.cliente;
        negocio.total_clientes += 1;
        cliente.id = negocio.total_clientes;
        cliente.dueno = ctx.accounts.dueno.key();
        cliente.nombre = nombre;
        cliente.compras = 0;
        cliente.ventas_activas = 0;
        cliente.activa = true;
        Ok(())
    }

    // 1 y 2. Crear Venta / Registrar venta válida
    pub fn crear_venta(ctx: Context<CrearVenta>, monto: u64) -> Result<()> {
        require!(ctx.accounts.cliente.activa, ErrorCode::ClienteInactivo);
        let negocio = &mut ctx.accounts.negocio;
        let cliente = &mut ctx.accounts.cliente;
        let venta = &mut ctx.accounts.venta;

        negocio.contador_ventas += 1;
        venta.id = negocio.contador_ventas;
        venta.cliente = cliente.key();
        venta.monto = monto;
        venta.cerrada = false;

        cliente.compras += 1;
        cliente.ventas_activas += 1;
        Ok(())
    }

    // 3. Corregir importe
    pub fn corregir_importe(ctx: Context<GestionarVenta>, nuevo_monto: u64) -> Result<()> {
        require!(!ctx.accounts.venta.cerrada, ErrorCode::VentaCerrada);
        ctx.accounts.venta.monto = nuevo_monto;
        Ok(())
    }

    // 4. Cerrar Venta / Devolución válida
    pub fn cerrar_venta(ctx: Context<GestionarVenta>) -> Result<()> {
        require!(!ctx.accounts.venta.cerrada, ErrorCode::VentaCerrada);
        let venta = &mut ctx.accounts.venta;
        let cliente = &mut ctx.accounts.cliente;
        venta.cerrada = true;
        cliente.ventas_activas = cliente.ventas_activas.saturating_sub(1);
        if cliente.ventas_activas == 0 {
            cliente.activa = false;
        }
        Ok(())
    }

    // 5, 7 y 8. Dar de baja cliente
    pub fn dar_de_baja_cliente(ctx: Context<DarDeBajaCliente>) -> Result<()> {
        // 7. La venta debe ser de ese cliente, si no, falla por has_one
        // 8. No se puede dar de baja con ventas activas
        require!(ctx.accounts.cliente.ventas_activas == 0, ErrorCode::TieneVentasActivas);
        ctx.accounts.cliente.activa = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InicializarNegocio<'info> {
    #[account(init, payer = autoridad, space = 8 + 32 + 8 + 8)]
    pub negocio: Account<'info, Negocio>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CrearCliente<'info> {
    #[account(mut)]
    pub negocio: Account<'info, Negocio>,
    #[account(init, payer = dueno, space = 8 + 8 + 32 + 32 + 8 + 8 + 1)]
    pub cliente: Account<'info, Cliente>,
    #[account(mut)]
    pub dueno: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CrearVenta<'info> {
    #[account(mut)]
    pub negocio: Account<'info, Negocio>,
    #[account(mut)]
    pub cliente: Account<'info, Cliente>,
    #[account(init, payer = autoridad, space = 8 + 8 + 32 + 8 + 1)]
    pub venta: Account<'info, Venta>,
    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GestionarVenta<'info> {
    // 6. Wallet no autorizada: solo el dueño del cliente puede gestionarla
    #[account(mut, has_one = dueno)]
    pub cliente: Account<'info, Cliente>,
    #[account(mut, has_one = cliente)]
    pub venta: Account<'info, Venta>,
    pub dueno: Signer<'info>,
}

#[derive(Accounts)]
pub struct DarDeBajaCliente<'info> {
    #[account(mut, has_one = dueno, close = dueno)]
    pub cliente: Account<'info, Cliente>,
    #[account(mut)]
    pub dueno: Signer<'info>,
}

#[account]
pub struct Negocio {
    pub autoridad: Pubkey,
    pub contador_ventas: u64,
    pub total_clientes: u64,
}

#[account]
pub struct Cliente {
    pub id: u64,
    pub dueno: Pubkey,
    pub nombre: String,
    pub compras: u64,
    pub ventas_activas: u64,
    pub activa: bool,
}

#[account]
pub struct Venta {
    pub id: u64,
    pub cliente: Pubkey,
    pub monto: u64,
    pub cerrada: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Cliente inactivo")]
    ClienteInactivo,
    #[msg("Venta ya cerrada")]
    VentaCerrada,
    #[msg("Cliente tiene ventas activas")]
    TieneVentasActivas,
}