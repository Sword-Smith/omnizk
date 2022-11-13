use c2zk_codegen_shared::CodegenError;
use c2zk_ir::ir;
use c2zk_ir::ir::Inst;

use crate::TritonTargetConfig;

#[allow(unused_variables)]
pub fn codegen(ins: &Inst, config: &TritonTargetConfig) -> Result<Vec<u8>, CodegenError> {
    match ins {
        ir::Inst::Unreachable => todo!(),
        ir::Inst::Nop => todo!(),
        ir::Inst::Block { ty } => todo!(),
        ir::Inst::Loop { ty } => todo!(),
        ir::Inst::If { ty } => todo!(),
        ir::Inst::Else => todo!(),
        ir::Inst::Try { ty } => todo!(),
        ir::Inst::Catch { index } => todo!(),
        ir::Inst::Throw { index } => todo!(),
        ir::Inst::Rethrow { relative_depth } => todo!(),
        ir::Inst::End => todo!(),
        ir::Inst::Br { relative_depth } => todo!(),
        ir::Inst::BrIf { relative_depth } => todo!(),
        ir::Inst::BrTable { table } => todo!(),
        ir::Inst::Return => todo!(),
        ir::Inst::Call { function_index } => todo!(),
        ir::Inst::CallIndirect { index, table_index } => todo!(),
        ir::Inst::ReturnCall { function_index } => todo!(),
        ir::Inst::ReturnCallIndirect { index, table_index } => todo!(),
        ir::Inst::Delegate { relative_depth } => todo!(),
        ir::Inst::CatchAll => todo!(),
        ir::Inst::Drop => todo!(),
        ir::Inst::Select => todo!(),
        ir::Inst::TypedSelect { ty } => todo!(),
        ir::Inst::LocalGet { local_index } => todo!(),
        ir::Inst::LocalSet { local_index } => todo!(),
        ir::Inst::LocalTee { local_index } => todo!(),
        ir::Inst::GlobalGet { global_index } => todo!(),
        ir::Inst::GlobalSet { global_index } => todo!(),
        ir::Inst::I32Load { memarg } => todo!(),
        ir::Inst::I64Load { memarg } => todo!(),
        ir::Inst::F32Load { memarg } => todo!(),
        ir::Inst::F64Load { memarg } => todo!(),
        ir::Inst::I32Load8S { memarg } => todo!(),
        ir::Inst::I32Load8U { memarg } => todo!(),
        ir::Inst::I32Load16S { memarg } => todo!(),
        ir::Inst::I32Load16U { memarg } => todo!(),
        ir::Inst::I64Load8S { memarg } => todo!(),
        ir::Inst::I64Load8U { memarg } => todo!(),
        ir::Inst::I64Load16S { memarg } => todo!(),
        ir::Inst::I64Load16U { memarg } => todo!(),
        ir::Inst::I64Load32S { memarg } => todo!(),
        ir::Inst::I64Load32U { memarg } => todo!(),
        ir::Inst::I32Store { memarg } => todo!(),
        ir::Inst::I64Store { memarg } => todo!(),
        ir::Inst::F32Store { memarg } => todo!(),
        ir::Inst::F64Store { memarg } => todo!(),
        ir::Inst::I32Store8 { memarg } => todo!(),
        ir::Inst::I32Store16 { memarg } => todo!(),
        ir::Inst::I64Store8 { memarg } => todo!(),
        ir::Inst::I64Store16 { memarg } => todo!(),
        ir::Inst::I64Store32 { memarg } => todo!(),
        ir::Inst::MemorySize { mem, mem_byte } => todo!(),
        ir::Inst::MemoryGrow { mem, mem_byte } => todo!(),
        ir::Inst::I32Const { value } => todo!(),
        ir::Inst::I64Const { value } => todo!(),
        ir::Inst::F32Const { value } => todo!(),
        ir::Inst::F64Const { value } => todo!(),
        ir::Inst::RefNull { ty } => todo!(),
        ir::Inst::RefIsNull => todo!(),
        ir::Inst::RefFunc { function_index } => todo!(),
        ir::Inst::I32Eqz => todo!(),
        ir::Inst::I32Eq => todo!(),
        ir::Inst::I32Ne => todo!(),
        ir::Inst::I32LtS => todo!(),
        ir::Inst::I32LtU => todo!(),
        ir::Inst::I32GtS => todo!(),
        ir::Inst::I32GtU => todo!(),
        ir::Inst::I32LeS => todo!(),
        ir::Inst::I32LeU => todo!(),
        ir::Inst::I32GeS => todo!(),
        ir::Inst::I32GeU => todo!(),
        ir::Inst::I64Eqz => todo!(),
        ir::Inst::I64Eq => todo!(),
        ir::Inst::I64Ne => todo!(),
        ir::Inst::I64LtS => todo!(),
        ir::Inst::I64LtU => todo!(),
        ir::Inst::I64GtS => todo!(),
        ir::Inst::I64GtU => todo!(),
        ir::Inst::I64LeS => todo!(),
        ir::Inst::I64LeU => todo!(),
        ir::Inst::I64GeS => todo!(),
        ir::Inst::I64GeU => todo!(),
        ir::Inst::F32Eq => todo!(),
        ir::Inst::F32Ne => todo!(),
        ir::Inst::F32Lt => todo!(),
        ir::Inst::F32Gt => todo!(),
        ir::Inst::F32Le => todo!(),
        ir::Inst::F32Ge => todo!(),
        ir::Inst::F64Eq => todo!(),
        ir::Inst::F64Ne => todo!(),
        ir::Inst::F64Lt => todo!(),
        ir::Inst::F64Gt => todo!(),
        ir::Inst::F64Le => todo!(),
        ir::Inst::F64Ge => todo!(),
        ir::Inst::I32Clz => todo!(),
        ir::Inst::I32Ctz => todo!(),
        ir::Inst::I32Popcnt => todo!(),
        ir::Inst::I32Add => todo!(),
        ir::Inst::I32Sub => todo!(),
        ir::Inst::I32Mul => todo!(),
        ir::Inst::I32DivS => todo!(),
        ir::Inst::I32DivU => todo!(),
        ir::Inst::I32RemS => todo!(),
        ir::Inst::I32RemU => todo!(),
        ir::Inst::I32And => todo!(),
        ir::Inst::I32Or => todo!(),
        ir::Inst::I32Xor => todo!(),
        ir::Inst::I32Shl => todo!(),
        ir::Inst::I32ShrS => todo!(),
        ir::Inst::I32ShrU => todo!(),
        ir::Inst::I32Rotl => todo!(),
        ir::Inst::I32Rotr => todo!(),
        ir::Inst::I64Clz => todo!(),
        ir::Inst::I64Ctz => todo!(),
        ir::Inst::I64Popcnt => todo!(),
        ir::Inst::I64Add => todo!(),
        ir::Inst::I64Sub => todo!(),
        ir::Inst::I64Mul => todo!(),
        ir::Inst::I64DivS => todo!(),
        ir::Inst::I64DivU => todo!(),
        ir::Inst::I64RemS => todo!(),
        ir::Inst::I64RemU => todo!(),
        ir::Inst::I64And => todo!(),
        ir::Inst::I64Or => todo!(),
        ir::Inst::I64Xor => todo!(),
        ir::Inst::I64Shl => todo!(),
        ir::Inst::I64ShrS => todo!(),
        ir::Inst::I64ShrU => todo!(),
        ir::Inst::I64Rotl => todo!(),
        ir::Inst::I64Rotr => todo!(),
        ir::Inst::F32Abs => todo!(),
        ir::Inst::F32Neg => todo!(),
        ir::Inst::F32Ceil => todo!(),
        ir::Inst::F32Floor => todo!(),
        ir::Inst::F32Trunc => todo!(),
        ir::Inst::F32Nearest => todo!(),
        ir::Inst::F32Sqrt => todo!(),
        ir::Inst::F32Add => todo!(),
        ir::Inst::F32Sub => todo!(),
        ir::Inst::F32Mul => todo!(),
        ir::Inst::F32Div => todo!(),
        ir::Inst::F32Min => todo!(),
        ir::Inst::F32Max => todo!(),
        ir::Inst::F32Copysign => todo!(),
        ir::Inst::F64Abs => todo!(),
        ir::Inst::F64Neg => todo!(),
        ir::Inst::F64Ceil => todo!(),
        ir::Inst::F64Floor => todo!(),
        ir::Inst::F64Trunc => todo!(),
        ir::Inst::F64Nearest => todo!(),
        ir::Inst::F64Sqrt => todo!(),
        ir::Inst::F64Add => todo!(),
        ir::Inst::F64Sub => todo!(),
        ir::Inst::F64Mul => todo!(),
        ir::Inst::F64Div => todo!(),
        ir::Inst::F64Min => todo!(),
        ir::Inst::F64Max => todo!(),
        ir::Inst::F64Copysign => todo!(),
        ir::Inst::I32WrapI64 => todo!(),
        ir::Inst::I32TruncF32S => todo!(),
        ir::Inst::I32TruncF32U => todo!(),
        ir::Inst::I32TruncF64S => todo!(),
        ir::Inst::I32TruncF64U => todo!(),
        ir::Inst::I64ExtendI32S => todo!(),
        ir::Inst::I64ExtendI32U => todo!(),
        ir::Inst::I64TruncF32S => todo!(),
        ir::Inst::I64TruncF32U => todo!(),
        ir::Inst::I64TruncF64S => todo!(),
        ir::Inst::I64TruncF64U => todo!(),
        ir::Inst::F32ConvertI32S => todo!(),
        ir::Inst::F32ConvertI32U => todo!(),
        ir::Inst::F32ConvertI64S => todo!(),
        ir::Inst::F32ConvertI64U => todo!(),
        ir::Inst::F32DemoteF64 => todo!(),
        ir::Inst::F64ConvertI32S => todo!(),
        ir::Inst::F64ConvertI32U => todo!(),
        ir::Inst::F64ConvertI64S => todo!(),
        ir::Inst::F64ConvertI64U => todo!(),
        ir::Inst::F64PromoteF32 => todo!(),
        ir::Inst::I32ReinterpretF32 => todo!(),
        ir::Inst::I64ReinterpretF64 => todo!(),
        ir::Inst::F32ReinterpretI32 => todo!(),
        ir::Inst::F64ReinterpretI64 => todo!(),
        ir::Inst::I32Extend8S => todo!(),
        ir::Inst::I32Extend16S => todo!(),
        ir::Inst::I64Extend8S => todo!(),
        ir::Inst::I64Extend16S => todo!(),
        ir::Inst::I64Extend32S => todo!(),
        ir::Inst::I32TruncSatF32S => todo!(),
        ir::Inst::I32TruncSatF32U => todo!(),
        ir::Inst::I32TruncSatF64S => todo!(),
        ir::Inst::I32TruncSatF64U => todo!(),
        ir::Inst::I64TruncSatF32S => todo!(),
        ir::Inst::I64TruncSatF32U => todo!(),
        ir::Inst::I64TruncSatF64S => todo!(),
        ir::Inst::I64TruncSatF64U => todo!(),
        ir::Inst::MemoryInit { segment, mem } => todo!(),
        ir::Inst::DataDrop { segment } => todo!(),
        ir::Inst::MemoryCopy { src, dst } => todo!(),
        ir::Inst::MemoryFill { mem } => todo!(),
        ir::Inst::TableInit { segment, table } => todo!(),
        ir::Inst::ElemDrop { segment } => todo!(),
        ir::Inst::TableCopy {
            dst_table,
            src_table,
        } => todo!(),
        ir::Inst::TableFill { table } => todo!(),
        ir::Inst::TableGet { table } => todo!(),
        ir::Inst::TableSet { table } => todo!(),
        ir::Inst::TableGrow { table } => todo!(),
        ir::Inst::TableSize { table } => todo!(),
        ir::Inst::MemoryAtomicNotify { memarg } => todo!(),
        ir::Inst::MemoryAtomicWait32 { memarg } => todo!(),
        ir::Inst::MemoryAtomicWait64 { memarg } => todo!(),
        ir::Inst::AtomicFence { flags } => todo!(),
        ir::Inst::I32AtomicLoad { memarg } => todo!(),
        ir::Inst::I64AtomicLoad { memarg } => todo!(),
        ir::Inst::I32AtomicLoad8U { memarg } => todo!(),
        ir::Inst::I32AtomicLoad16U { memarg } => todo!(),
        ir::Inst::I64AtomicLoad8U { memarg } => todo!(),
        ir::Inst::I64AtomicLoad16U { memarg } => todo!(),
        ir::Inst::I64AtomicLoad32U { memarg } => todo!(),
        ir::Inst::I32AtomicStore { memarg } => todo!(),
        ir::Inst::I64AtomicStore { memarg } => todo!(),
        ir::Inst::I32AtomicStore8 { memarg } => todo!(),
        ir::Inst::I32AtomicStore16 { memarg } => todo!(),
        ir::Inst::I64AtomicStore8 { memarg } => todo!(),
        ir::Inst::I64AtomicStore16 { memarg } => todo!(),
        ir::Inst::I64AtomicStore32 { memarg } => todo!(),
        ir::Inst::I32AtomicRmwAdd { memarg } => todo!(),
        ir::Inst::I64AtomicRmwAdd { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8AddU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16AddU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8AddU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16AddU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32AddU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwSub { memarg } => todo!(),
        ir::Inst::I64AtomicRmwSub { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8SubU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16SubU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8SubU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16SubU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32SubU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwAnd { memarg } => todo!(),
        ir::Inst::I64AtomicRmwAnd { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8AndU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16AndU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8AndU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16AndU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32AndU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwOr { memarg } => todo!(),
        ir::Inst::I64AtomicRmwOr { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8OrU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16OrU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8OrU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16OrU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32OrU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwXor { memarg } => todo!(),
        ir::Inst::I64AtomicRmwXor { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8XorU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16XorU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8XorU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16XorU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32XorU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwXchg { memarg } => todo!(),
        ir::Inst::I64AtomicRmwXchg { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8XchgU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16XchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8XchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16XchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32XchgU { memarg } => todo!(),
        ir::Inst::I32AtomicRmwCmpxchg { memarg } => todo!(),
        ir::Inst::I64AtomicRmwCmpxchg { memarg } => todo!(),
        ir::Inst::I32AtomicRmw8CmpxchgU { memarg } => todo!(),
        ir::Inst::I32AtomicRmw16CmpxchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw8CmpxchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw16CmpxchgU { memarg } => todo!(),
        ir::Inst::I64AtomicRmw32CmpxchgU { memarg } => todo!(),
        ir::Inst::V128Load { memarg } => todo!(),
        ir::Inst::V128Load8x8S { memarg } => todo!(),
        ir::Inst::V128Load8x8U { memarg } => todo!(),
        ir::Inst::V128Load16x4S { memarg } => todo!(),
        ir::Inst::V128Load16x4U { memarg } => todo!(),
        ir::Inst::V128Load32x2S { memarg } => todo!(),
        ir::Inst::V128Load32x2U { memarg } => todo!(),
        ir::Inst::V128Load8Splat { memarg } => todo!(),
        ir::Inst::V128Load16Splat { memarg } => todo!(),
        ir::Inst::V128Load32Splat { memarg } => todo!(),
        ir::Inst::V128Load64Splat { memarg } => todo!(),
        ir::Inst::V128Load32Zero { memarg } => todo!(),
        ir::Inst::V128Load64Zero { memarg } => todo!(),
        ir::Inst::V128Store { memarg } => todo!(),
        ir::Inst::V128Load8Lane { memarg, lane } => todo!(),
        ir::Inst::V128Load16Lane { memarg, lane } => todo!(),
        ir::Inst::V128Load32Lane { memarg, lane } => todo!(),
        ir::Inst::V128Load64Lane { memarg, lane } => todo!(),
        ir::Inst::V128Store8Lane { memarg, lane } => todo!(),
        ir::Inst::V128Store16Lane { memarg, lane } => todo!(),
        ir::Inst::V128Store32Lane { memarg, lane } => todo!(),
        ir::Inst::V128Store64Lane { memarg, lane } => todo!(),
        ir::Inst::V128Const { value } => todo!(),
        ir::Inst::I8x16Shuffle { lanes } => todo!(),
        ir::Inst::I8x16ExtractLaneS { lane } => todo!(),
        ir::Inst::I8x16ExtractLaneU { lane } => todo!(),
        ir::Inst::I8x16ReplaceLane { lane } => todo!(),
        ir::Inst::I16x8ExtractLaneS { lane } => todo!(),
        ir::Inst::I16x8ExtractLaneU { lane } => todo!(),
        ir::Inst::I16x8ReplaceLane { lane } => todo!(),
        ir::Inst::I32x4ExtractLane { lane } => todo!(),
        ir::Inst::I32x4ReplaceLane { lane } => todo!(),
        ir::Inst::I64x2ExtractLane { lane } => todo!(),
        ir::Inst::I64x2ReplaceLane { lane } => todo!(),
        ir::Inst::F32x4ExtractLane { lane } => todo!(),
        ir::Inst::F32x4ReplaceLane { lane } => todo!(),
        ir::Inst::F64x2ExtractLane { lane } => todo!(),
        ir::Inst::F64x2ReplaceLane { lane } => todo!(),
        ir::Inst::I8x16Swizzle => todo!(),
        ir::Inst::I8x16Splat => todo!(),
        ir::Inst::I16x8Splat => todo!(),
        ir::Inst::I32x4Splat => todo!(),
        ir::Inst::I64x2Splat => todo!(),
        ir::Inst::F32x4Splat => todo!(),
        ir::Inst::F64x2Splat => todo!(),
        ir::Inst::I8x16Eq => todo!(),
        ir::Inst::I8x16Ne => todo!(),
        ir::Inst::I8x16LtS => todo!(),
        ir::Inst::I8x16LtU => todo!(),
        ir::Inst::I8x16GtS => todo!(),
        ir::Inst::I8x16GtU => todo!(),
        ir::Inst::I8x16LeS => todo!(),
        ir::Inst::I8x16LeU => todo!(),
        ir::Inst::I8x16GeS => todo!(),
        ir::Inst::I8x16GeU => todo!(),
        ir::Inst::I16x8Eq => todo!(),
        ir::Inst::I16x8Ne => todo!(),
        ir::Inst::I16x8LtS => todo!(),
        ir::Inst::I16x8LtU => todo!(),
        ir::Inst::I16x8GtS => todo!(),
        ir::Inst::I16x8GtU => todo!(),
        ir::Inst::I16x8LeS => todo!(),
        ir::Inst::I16x8LeU => todo!(),
        ir::Inst::I16x8GeS => todo!(),
        ir::Inst::I16x8GeU => todo!(),
        ir::Inst::I32x4Eq => todo!(),
        ir::Inst::I32x4Ne => todo!(),
        ir::Inst::I32x4LtS => todo!(),
        ir::Inst::I32x4LtU => todo!(),
        ir::Inst::I32x4GtS => todo!(),
        ir::Inst::I32x4GtU => todo!(),
        ir::Inst::I32x4LeS => todo!(),
        ir::Inst::I32x4LeU => todo!(),
        ir::Inst::I32x4GeS => todo!(),
        ir::Inst::I32x4GeU => todo!(),
        ir::Inst::I64x2Eq => todo!(),
        ir::Inst::I64x2Ne => todo!(),
        ir::Inst::I64x2LtS => todo!(),
        ir::Inst::I64x2GtS => todo!(),
        ir::Inst::I64x2LeS => todo!(),
        ir::Inst::I64x2GeS => todo!(),
        ir::Inst::F32x4Eq => todo!(),
        ir::Inst::F32x4Ne => todo!(),
        ir::Inst::F32x4Lt => todo!(),
        ir::Inst::F32x4Gt => todo!(),
        ir::Inst::F32x4Le => todo!(),
        ir::Inst::F32x4Ge => todo!(),
        ir::Inst::F64x2Eq => todo!(),
        ir::Inst::F64x2Ne => todo!(),
        ir::Inst::F64x2Lt => todo!(),
        ir::Inst::F64x2Gt => todo!(),
        ir::Inst::F64x2Le => todo!(),
        ir::Inst::F64x2Ge => todo!(),
        ir::Inst::V128Not => todo!(),
        ir::Inst::V128And => todo!(),
        ir::Inst::V128AndNot => todo!(),
        ir::Inst::V128Or => todo!(),
        ir::Inst::V128Xor => todo!(),
        ir::Inst::V128Bitselect => todo!(),
        ir::Inst::V128AnyTrue => todo!(),
        ir::Inst::I8x16Abs => todo!(),
        ir::Inst::I8x16Neg => todo!(),
        ir::Inst::I8x16Popcnt => todo!(),
        ir::Inst::I8x16AllTrue => todo!(),
        ir::Inst::I8x16Bitmask => todo!(),
        ir::Inst::I8x16NarrowI16x8S => todo!(),
        ir::Inst::I8x16NarrowI16x8U => todo!(),
        ir::Inst::I8x16Shl => todo!(),
        ir::Inst::I8x16ShrS => todo!(),
        ir::Inst::I8x16ShrU => todo!(),
        ir::Inst::I8x16Add => todo!(),
        ir::Inst::I8x16AddSatS => todo!(),
        ir::Inst::I8x16AddSatU => todo!(),
        ir::Inst::I8x16Sub => todo!(),
        ir::Inst::I8x16SubSatS => todo!(),
        ir::Inst::I8x16SubSatU => todo!(),
        ir::Inst::I8x16MinS => todo!(),
        ir::Inst::I8x16MinU => todo!(),
        ir::Inst::I8x16MaxS => todo!(),
        ir::Inst::I8x16MaxU => todo!(),
        ir::Inst::I8x16RoundingAverageU => todo!(),
        ir::Inst::I16x8ExtAddPairwiseI8x16S => todo!(),
        ir::Inst::I16x8ExtAddPairwiseI8x16U => todo!(),
        ir::Inst::I16x8Abs => todo!(),
        ir::Inst::I16x8Neg => todo!(),
        ir::Inst::I16x8Q15MulrSatS => todo!(),
        ir::Inst::I16x8AllTrue => todo!(),
        ir::Inst::I16x8Bitmask => todo!(),
        ir::Inst::I16x8NarrowI32x4S => todo!(),
        ir::Inst::I16x8NarrowI32x4U => todo!(),
        ir::Inst::I16x8ExtendLowI8x16S => todo!(),
        ir::Inst::I16x8ExtendHighI8x16S => todo!(),
        ir::Inst::I16x8ExtendLowI8x16U => todo!(),
        ir::Inst::I16x8ExtendHighI8x16U => todo!(),
        ir::Inst::I16x8Shl => todo!(),
        ir::Inst::I16x8ShrS => todo!(),
        ir::Inst::I16x8ShrU => todo!(),
        ir::Inst::I16x8Add => todo!(),
        ir::Inst::I16x8AddSatS => todo!(),
        ir::Inst::I16x8AddSatU => todo!(),
        ir::Inst::I16x8Sub => todo!(),
        ir::Inst::I16x8SubSatS => todo!(),
        ir::Inst::I16x8SubSatU => todo!(),
        ir::Inst::I16x8Mul => todo!(),
        ir::Inst::I16x8MinS => todo!(),
        ir::Inst::I16x8MinU => todo!(),
        ir::Inst::I16x8MaxS => todo!(),
        ir::Inst::I16x8MaxU => todo!(),
        ir::Inst::I16x8RoundingAverageU => todo!(),
        ir::Inst::I16x8ExtMulLowI8x16S => todo!(),
        ir::Inst::I16x8ExtMulHighI8x16S => todo!(),
        ir::Inst::I16x8ExtMulLowI8x16U => todo!(),
        ir::Inst::I16x8ExtMulHighI8x16U => todo!(),
        ir::Inst::I32x4ExtAddPairwiseI16x8S => todo!(),
        ir::Inst::I32x4ExtAddPairwiseI16x8U => todo!(),
        ir::Inst::I32x4Abs => todo!(),
        ir::Inst::I32x4Neg => todo!(),
        ir::Inst::I32x4AllTrue => todo!(),
        ir::Inst::I32x4Bitmask => todo!(),
        ir::Inst::I32x4ExtendLowI16x8S => todo!(),
        ir::Inst::I32x4ExtendHighI16x8S => todo!(),
        ir::Inst::I32x4ExtendLowI16x8U => todo!(),
        ir::Inst::I32x4ExtendHighI16x8U => todo!(),
        ir::Inst::I32x4Shl => todo!(),
        ir::Inst::I32x4ShrS => todo!(),
        ir::Inst::I32x4ShrU => todo!(),
        ir::Inst::I32x4Add => todo!(),
        ir::Inst::I32x4Sub => todo!(),
        ir::Inst::I32x4Mul => todo!(),
        ir::Inst::I32x4MinS => todo!(),
        ir::Inst::I32x4MinU => todo!(),
        ir::Inst::I32x4MaxS => todo!(),
        ir::Inst::I32x4MaxU => todo!(),
        ir::Inst::I32x4DotI16x8S => todo!(),
        ir::Inst::I32x4ExtMulLowI16x8S => todo!(),
        ir::Inst::I32x4ExtMulHighI16x8S => todo!(),
        ir::Inst::I32x4ExtMulLowI16x8U => todo!(),
        ir::Inst::I32x4ExtMulHighI16x8U => todo!(),
        ir::Inst::I64x2Abs => todo!(),
        ir::Inst::I64x2Neg => todo!(),
        ir::Inst::I64x2AllTrue => todo!(),
        ir::Inst::I64x2Bitmask => todo!(),
        ir::Inst::I64x2ExtendLowI32x4S => todo!(),
        ir::Inst::I64x2ExtendHighI32x4S => todo!(),
        ir::Inst::I64x2ExtendLowI32x4U => todo!(),
        ir::Inst::I64x2ExtendHighI32x4U => todo!(),
        ir::Inst::I64x2Shl => todo!(),
        ir::Inst::I64x2ShrS => todo!(),
        ir::Inst::I64x2ShrU => todo!(),
        ir::Inst::I64x2Add => todo!(),
        ir::Inst::I64x2Sub => todo!(),
        ir::Inst::I64x2Mul => todo!(),
        ir::Inst::I64x2ExtMulLowI32x4S => todo!(),
        ir::Inst::I64x2ExtMulHighI32x4S => todo!(),
        ir::Inst::I64x2ExtMulLowI32x4U => todo!(),
        ir::Inst::I64x2ExtMulHighI32x4U => todo!(),
        ir::Inst::F32x4Ceil => todo!(),
        ir::Inst::F32x4Floor => todo!(),
        ir::Inst::F32x4Trunc => todo!(),
        ir::Inst::F32x4Nearest => todo!(),
        ir::Inst::F32x4Abs => todo!(),
        ir::Inst::F32x4Neg => todo!(),
        ir::Inst::F32x4Sqrt => todo!(),
        ir::Inst::F32x4Add => todo!(),
        ir::Inst::F32x4Sub => todo!(),
        ir::Inst::F32x4Mul => todo!(),
        ir::Inst::F32x4Div => todo!(),
        ir::Inst::F32x4Min => todo!(),
        ir::Inst::F32x4Max => todo!(),
        ir::Inst::F32x4PMin => todo!(),
        ir::Inst::F32x4PMax => todo!(),
        ir::Inst::F64x2Ceil => todo!(),
        ir::Inst::F64x2Floor => todo!(),
        ir::Inst::F64x2Trunc => todo!(),
        ir::Inst::F64x2Nearest => todo!(),
        ir::Inst::F64x2Abs => todo!(),
        ir::Inst::F64x2Neg => todo!(),
        ir::Inst::F64x2Sqrt => todo!(),
        ir::Inst::F64x2Add => todo!(),
        ir::Inst::F64x2Sub => todo!(),
        ir::Inst::F64x2Mul => todo!(),
        ir::Inst::F64x2Div => todo!(),
        ir::Inst::F64x2Min => todo!(),
        ir::Inst::F64x2Max => todo!(),
        ir::Inst::F64x2PMin => todo!(),
        ir::Inst::F64x2PMax => todo!(),
        ir::Inst::I32x4TruncSatF32x4S => todo!(),
        ir::Inst::I32x4TruncSatF32x4U => todo!(),
        ir::Inst::F32x4ConvertI32x4S => todo!(),
        ir::Inst::F32x4ConvertI32x4U => todo!(),
        ir::Inst::I32x4TruncSatF64x2SZero => todo!(),
        ir::Inst::I32x4TruncSatF64x2UZero => todo!(),
        ir::Inst::F64x2ConvertLowI32x4S => todo!(),
        ir::Inst::F64x2ConvertLowI32x4U => todo!(),
        ir::Inst::F32x4DemoteF64x2Zero => todo!(),
        ir::Inst::F64x2PromoteLowF32x4 => todo!(),
        ir::Inst::I8x16RelaxedSwizzle => todo!(),
        ir::Inst::I32x4RelaxedTruncSatF32x4S => todo!(),
        ir::Inst::I32x4RelaxedTruncSatF32x4U => todo!(),
        ir::Inst::I32x4RelaxedTruncSatF64x2SZero => todo!(),
        ir::Inst::I32x4RelaxedTruncSatF64x2UZero => todo!(),
        ir::Inst::F32x4Fma => todo!(),
        ir::Inst::F32x4Fms => todo!(),
        ir::Inst::F64x2Fma => todo!(),
        ir::Inst::F64x2Fms => todo!(),
        ir::Inst::I8x16LaneSelect => todo!(),
        ir::Inst::I16x8LaneSelect => todo!(),
        ir::Inst::I32x4LaneSelect => todo!(),
        ir::Inst::I64x2LaneSelect => todo!(),
        ir::Inst::F32x4RelaxedMin => todo!(),
        ir::Inst::F32x4RelaxedMax => todo!(),
        ir::Inst::F64x2RelaxedMin => todo!(),
        ir::Inst::F64x2RelaxedMax => todo!(),
    }
    Ok(vec![])
}
