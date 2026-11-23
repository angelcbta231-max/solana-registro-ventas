# Solana - Registro de Ventas

Proyecto para registrar ventas en la blockchain de Solana.

## ¿Qué hace?
Cada venta guarda:
- Monto de la venta
- Vendedor
- Fecha

## Reglas
1. El monto no puede ser cero.
2. No se puede registrar la misma venta dos veces.

## Archivos
- `src/lib.rs` - Programa en Rust, aquí están las reglas.
- `client/client.ts` - Cliente que manda las ventas.
- `tests/anchor.test.ts` - Pruebas del programa.

## ¿Cómo probarlo?
1. Abrir el proyecto en Solana Playground.
2. Dar clic en Build.
3. Dar clic en Test.

## Resultado
8 passing - Todas las pruebas pasan, incluyendo las 2 que verifican que el programa bloquea montos en cero y ventas duplicadas.