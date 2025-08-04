
Pipeline Implementation Summary
===============================

Modules Generated:
1. pipelined_adder.v     - High throughput adder (II=1, Depth=8)
2. dsp_multiplier.v      - DSP-optimized multiplier (II=1, Depth=6)  
3. pipelined_mac.v       - Balanced MAC unit (II=2, Depth=4)
4. low_latency_sub.v     - Low latency subtractor (II=4, Depth=2)
5. simple_reference.v    - Non-pipelined reference

Pipeline Metrics:
- High Throughput: Best for streaming applications
- DSP Intensive: Optimized for multiply-heavy workloads
- Balanced: Good latency/throughput trade-off
- Low Latency: Minimal delay for critical paths

Vivado Integration:
1. Create new project targeting your FPGA device
2. Add all .v files as design sources
3. Set top-level module as needed
4. Run synthesis and implementation
5. Check timing reports for achieved frequency

AMD Alveo U50 Expected Performance:
- High throughput modules: 400+ MHz
- DSP intensive modules: 500+ MHz (with DSP48 primitives)
- Low latency modules: 600+ MHz
