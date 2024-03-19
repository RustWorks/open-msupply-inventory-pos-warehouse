import { useGenerateInboundReturnLines } from './useGenerateInboundReturnLines';
import { useDeleteSelectedInboundReturnLines } from './useDeleteSelectedInboundLines';
import { useInboundReturnRows } from './useInboundReturnRows';
import { useOutboundReturnLines } from './useOutboundReturnLines';
import { useUpdateOutboundReturnLines } from './useUpdateOutboundReturnLines';

export const Lines = {
  useOutboundReturnLines,
  useUpdateOutboundReturnLines,

  useGenerateInboundReturnLines,
  useInboundReturnRows,
  useDeleteSelectedInboundReturnLines,
};