use std::collections::VecDeque;
use std::io::Write;
use std::sync::atomic::AtomicIsize;
use std::sync::Arc;
use std::time::Instant;
use vulkano::descriptor_set::WriteDescriptorSet;

use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer, ImmutableBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage};
use vulkano::descriptor_set::{DescriptorSet, PersistentDescriptorSet};
use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceExtensions, Features};
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::pipeline::{ComputePipeline, Pipeline, PipelineBindPoint};
use vulkano::sync;
use vulkano::sync::GpuFuture;
use vulkano::Version;

use crate::traits::*;

pub struct S;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
struct Register(u8);

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
enum Operand {
    Register(Register),
    Literal(i8),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ins {
    Inp(Register),
    Add(Register, Operand),
    Mul(Register, Operand),
    Div(Register, Operand),
    Mod(Register, Operand),
    Eql(Register, Operand),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum InstructionOr {
    Ins(Ins),
    KnownValue(isize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SideEffectTree {
    ins: InstructionOr,
    index: usize,
    parents: HashMap<Register, SideEffectTree>,
}

#[derive(Debug, Clone)]
struct Program(Vec<Ins>);

struct Computer([isize; 4]);

impl Register {
    fn parse<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let s = items.next().unwrap();
        match s {
            "w" => Register(0),
            "x" => Register(1),
            "y" => Register(2),
            "z" => Register(3),
            _ => panic!("Unexpected {}", s),
        }
    }

    fn write(self, computer: &mut Computer, value: isize) {
        computer.0[self.0 as usize] = value;
    }

    fn read(self, computer: &Computer) -> isize {
        computer.0[self.0 as usize]
    }
}

impl std::fmt::Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self.0 {
            0 => "w",
            1 => "x",
            2 => "y",
            3 => "z",
            _ => unreachable!(),
        };
        f.write_str(c)
    }
}

impl Ins {
    fn get_modified_register(self) -> Register {
        match self {
            Ins::Inp(a) => a,
            Ins::Add(a, _b) => a,
            Ins::Mul(a, _b) => a,
            Ins::Div(a, _b) => a,
            Ins::Mod(a, _b) => a,
            Ins::Eql(a, _b) => a,
        }
    }

    fn get_read_write_registers(self) -> Vec<Register> {
        fn combine_args(a: Register, b: Operand) -> Vec<Register> {
            match b {
                Operand::Literal(_lit) => vec![a],
                Operand::Register(reg) => vec![a, reg],
            }
        }
        match self {
            Ins::Inp(a) => vec![a],
            Ins::Add(a, b) => combine_args(a, b),
            Ins::Mul(a, b) => combine_args(a, b),
            Ins::Div(a, b) => combine_args(a, b),
            Ins::Mod(a, b) => combine_args(a, b),
            Ins::Eql(a, b) => combine_args(a, b),
        }
    }
}

impl std::fmt::Display for Ins {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ins::Inp(a) => f.write_fmt(format_args!("inp {}", a)),
            Ins::Add(a, b) => f.write_fmt(format_args!("add {} {}", a, b)),
            Ins::Mul(a, b) => f.write_fmt(format_args!("mul {} {}", a, b)),
            Ins::Div(a, b) => f.write_fmt(format_args!("div {} {}", a, b)),
            Ins::Mod(a, b) => f.write_fmt(format_args!("mod {} {}", a, b)),
            Ins::Eql(a, b) => f.write_fmt(format_args!("eql {} {}", a, b)),
        }
    }
}

impl std::fmt::Display for InstructionOr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            InstructionOr::Ins(ins) => ins.fmt(f),
            InstructionOr::KnownValue(value) => value.fmt(f),
        }
    }
}

impl Operand {
    fn parse<'a>(items: &mut impl Iterator<Item = &'a str>) -> Self {
        let s = items.next().unwrap();
        match s {
            "w" => Operand::Register(Register(0)),
            "x" => Operand::Register(Register(1)),
            "y" => Operand::Register(Register(2)),
            "z" => Operand::Register(Register(3)),
            _ => Operand::Literal(s.parse().unwrap()),
        }
    }

    fn read(self, computer: &Computer) -> isize {
        match self {
            Operand::Register(reg) => computer.0[reg.0 as usize],
            Operand::Literal(lit) => lit as isize,
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Literal(lit) => f.write_fmt(format_args!("{}", lit)),
            Operand::Register(reg) => f.write_fmt(format_args!("{}", reg)),
        }
    }
}

impl Program {
    fn parse(input: Input) -> Program {
        let program = input
            .lines()
            .map(|line| {
                let mut parts = line.split(' ');
                let ins = parts.next().unwrap();
                match ins {
                    "inp" => Ins::Inp(Register::parse(&mut parts)),
                    "add" => Ins::Add(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "mul" => Ins::Mul(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "div" => Ins::Div(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "mod" => Ins::Mod(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    "eql" => Ins::Eql(Register::parse(&mut parts), Operand::parse(&mut parts)),
                    bad => unreachable!("{}", bad),
                }
            })
            .collect();

        Program(program)
    }
}

impl Computer {
    fn new() -> Self {
        Self([0; 4])
    }

    fn is_valid(&mut self, program: &Program, serial: &[u8]) -> bool {
        if serial.contains(&0) {
            return false;
        }
        let mut s_index = 0;
        for ins in &program.0 {
            match ins {
                Ins::Inp(op) => {
                    op.write(self, (serial[s_index] as isize) - (b'0' as isize));
                    s_index += 1;
                }
                Ins::Add(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a + b);
                }
                Ins::Mul(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a * b);
                }
                Ins::Div(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a / b);
                }
                Ins::Mod(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    op_a.write(self, a % b);
                }
                Ins::Eql(op_a, op_b) => {
                    let a = op_a.read(self);
                    let b = op_b.read(self);
                    let result = if a == b { 1 } else { 0 };
                    op_a.write(self, result);
                }
            }
        }

        self.0[3] == 0
    }
}

static SERIAL: AtomicIsize = AtomicIsize::new(99_999_999_999_999);

impl AocDay for S {
    fn part1(&self, input: Input) -> Output {
        let program = Program::parse(input);
        //let (side_effects, possible_inputs) = program.build_side_effects(Some(100));
        //println!("{}", side_effects.print_tree());
        let input_count = program
            .0
            .iter()
            .filter_map(|i| if let Ins::Inp(_) = i { Some(()) } else { None })
            .count();

        let mut shader_src = Vec::new();
        const NUM_GROUPS: usize = 15625;
        const LOCAL_SIZE: usize = 64;
        const DISPATCH_COUNT: usize = NUM_GROUPS * LOCAL_SIZE;
        //This must be a power of 10 for our math to work out...
        assert_eq!(f64::log10(DISPATCH_COUNT as f64) % 1.0, 0.0);

        const SERIALS_PER_THREAD: usize = 1000;
        const SERIALS_PER_DISPATCH: usize = SERIALS_PER_THREAD * DISPATCH_COUNT;
        program.write_to_spv(SERIALS_PER_THREAD, LOCAL_SIZE, &mut shader_src);

        //Run spriv compiler
        let mut glslc_process = std::process::Command::new("glslc")
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .stdin(std::process::Stdio::piped())
            .arg("-fshader-stage=compute")
            .arg("-O")
            .arg("-o")
            .arg("-")
            .arg("-")
            .spawn()
            .unwrap();

        let mut stdin = glslc_process.stdin.take().unwrap();
        println!("Shader:");
        for (i, line) in std::str::from_utf8(&shader_src)
            .unwrap()
            .lines()
            .enumerate()
        {
            println!("{: >3}|{}", i, line);
        }
        stdin.write_all(shader_src.as_slice()).unwrap();
        drop(stdin);

        let glslc_out = glslc_process.wait_with_output().unwrap();
        if !glslc_out.status.success() {
            panic!(
                "glslc failed: {}",
                String::from_utf8_lossy(glslc_out.stderr.as_slice())
            );
        }
        let spv_binary = glslc_out.stdout;

        // As with other examples, the first step is to create an instance.
        let instance =
            Instance::new(None, Version::V1_1, &InstanceExtensions::none(), None).unwrap();

        let _callback =
            vulkano::instance::debug::DebugCallback::errors_and_warnings(&instance, |msg| {
                println!("Debug CB: {:?}", msg.description);
            })
            .ok();

        // Choose which physical device to use.
        let device_extensions = DeviceExtensions {
            khr_storage_buffer_storage_class: true,
            ..DeviceExtensions::none()
        };
        let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
            .filter_map(|p| {
                // The Vulkan specs guarantee that a compliant implementation must provide at least one queue
                // that supports compute operations.
                p.queue_families()
                    .find(|&q| q.supports_compute())
                    .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
            })
            .unwrap();

        println!(
            "Using device: {} (type: {:?})",
            physical_device.properties().device_name,
            physical_device.properties().device_type
        );

        let features = Features::none();

        // Now initializing the device.
        let (device, mut queues) = Device::new(
            physical_device,
            &features,
            &physical_device
                .required_extensions()
                .union(&device_extensions),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        // Since we can request multiple queues, the `queues` variable is in fact an iterator. In this
        // example we use only one queue, so we just retrieve the first and only element of the
        // iterator and throw it away.
        let queue = queues.next().unwrap();

        // Now let's get to the actual example.
        //
        // What we are going to do is very basic: we are going to fill a buffer with 64k integers
        // and ask the GPU to multiply each of them by 12.
        //
        // GPUs are very good at parallel computations (SIMD-like operations), and thus will do this
        // much more quickly than a CPU would do. While a CPU would typically multiply them one by one
        // or four by four, a GPU will do it by groups of 32 or 64.
        //
        // Note however that in a real-life situation for such a simple operation the cost of
        // accessing memory usually outweighs the benefits of a faster calculation. Since both the CPU
        // and the GPU will need to access data, there is no other choice but to transfer the data
        // through the slow PCI express bus.

        // We need to create the compute pipeline that describes our operation.
        //
        // If you are familiar with graphics pipeline, the principle is the same except that compute
        // pipelines are much simpler to create.
        let pipeline = {
            let shader = unsafe {
                vulkano::shader::ShaderModule::from_bytes(Arc::clone(&device), &spv_binary)
            }
            .unwrap();

            ComputePipeline::new(
                device.clone(),
                shader.entry_point("main").unwrap(),
                &(),
                None,
                |_| {},
            )
            .unwrap()
        };

        let mut current_serial = 71_999_999_999_999usize;

        let mut last_start = Instant::now();
        while current_serial > 10_000_000_000_000 {
            println!("Ruining {} - {:?}", current_serial, last_start.elapsed());
            last_start = Instant::now();
            let in_buffer = {
                // Iterator that produces the data.
                let data_iter: Vec<[u8; 16]> = (0..)
                    .into_iter()
                    .map(|i| current_serial - SERIALS_PER_THREAD * i)
                    .filter_map(|n| {
                        let mut base10 = format!("{:0>14}", n);
                        if base10.contains('0') {
                            None
                        } else {
                            base10.push_str("00");
                            let mut vec: Vec<u8> = base10.into();
                            vec.iter_mut().for_each(|a| *a -= b'0');
                            let mut dst = [0u8; 16];
                            dst.copy_from_slice(&vec);
                            Some(dst)
                        }
                    })
                    .take(DISPATCH_COUNT)
                    .collect();

                // Builds the buffer and fills it with this iterator.
                CpuAccessibleBuffer::from_iter(
                    Arc::clone(&device),
                    BufferUsage {
                        storage_buffer: true,
                        ..BufferUsage::none()
                    },
                    false,
                    data_iter,
                )
                .unwrap()
            };

            // In order to let the shader access the buffer, we need to build a *descriptor set* that
            // contains the buffer.
            //
            // The resources that we bind to the descriptor set must match the resources expected by the
            // pipeline which we pass as the first parameter.
            //
            // If you want to run the pipeline on multiple different buffers, you need to create multiple
            // descriptor sets that each contain the buffer you want to run the shader on.
            let layouts = pipeline.layout().descriptor_set_layouts();
            let layout = layouts.get(0).unwrap();
            let set = PersistentDescriptorSet::new(
                layout.clone(),
                [WriteDescriptorSet::buffer(0, in_buffer.clone())],
            )
            .unwrap();

            //let mut current_serial = 98_765_432_198_766usize;
            // We start by creating the buffer that will store the data.
            let in_buffer = {
                // Iterator that produces the data.
                let data_iter: Vec<[u8; 16]> = (0..)
                    .into_iter()
                    .map(|i| current_serial - SERIALS_PER_THREAD * i)
                    .filter_map(|n| {
                        let mut base10 = format!("{:0>14}", n);
                        if base10.contains('0') {
                            None
                        } else {
                            base10.push_str("00");
                            let mut vec: Vec<u8> = base10.into();
                            vec.iter_mut().for_each(|a| *a -= b'0');
                            let mut dst = [0u8; 16];
                            dst.copy_from_slice(&vec);
                            Some(dst)
                        }
                    })
                    .take(DISPATCH_COUNT)
                    .collect();

                // Builds the buffer and fills it with this iterator.
                CpuAccessibleBuffer::from_iter(
                    Arc::clone(&device),
                    BufferUsage {
                        storage_buffer: true,
                        ..BufferUsage::none()
                    },
                    false,
                    data_iter,
                )
                .unwrap()
            };

            current_serial -= SERIALS_PER_DISPATCH;

            // In order to execute our operation, we have to build a command buffer.
            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.family(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();

            builder
                // The command buffer only does one thing: execute the compute pipeline.
                // This is called a *dispatch* operation.
                //
                // Note that we clone the pipeline and the set. Since they are both wrapped around an
                // `Arc`, this only clones the `Arc` and not the whole pipeline or set (which aren't
                // cloneable anyway). In this example we would avoid cloning them since this is the last
                // time we use them, but in a real code you would probably need to clone them.
                .bind_pipeline_compute(Arc::clone(&pipeline))
                .bind_descriptor_sets(
                    PipelineBindPoint::Compute,
                    Arc::clone(pipeline.layout()),
                    0,
                    set.clone(),
                )
                .dispatch([NUM_GROUPS as u32, 1, 1])
                .unwrap();
            // Finish building the command buffer by calling `build`.
            let command_buffer = builder.build().unwrap();

            // Let's execute this command buffer now.
            // To do so, we TODO: this is a bit clumsy, probably needs a shortcut
            let future = sync::now(Arc::clone(&device))
                .then_execute(Arc::clone(&queue), command_buffer)
                .unwrap()
                // This line instructs the GPU to signal a *fence* once the command buffer has finished
                // execution. A fence is a Vulkan object that allows the CPU to know when the GPU has
                // reached a certain point.
                // We need to signal a fence here because below we want to block the CPU until the GPU has
                // reached that point in the execution.
                .then_signal_fence_and_flush()
                .unwrap();

            // Blocks execution until the GPU has finished the operation. This method only exists on the
            // future that corresponds to a signalled fence. In other words, this method wouldn't be
            // available if we didn't call `.then_signal_fence_and_flush()` earlier.
            // The `None` parameter is an optional timeout.
            //
            // Note however that dropping the `future` variable (with `drop(future)` for example) would
            // block execution as well, and this would be the case even if we didn't call
            // `.then_signal_fence_and_flush()`.
            // Therefore the actual point of calling `.then_signal_fence_and_flush()` and `.wait()` is to
            // make things more explicit. In the future, if the Rust language gets linear types vulkano may
            // get modified so that only fence-signalled futures can get destroyed like this.
            future.wait(None).unwrap();

            // Now that the GPU is done, the content of the buffer should have been modified. Let's
            // check it out.
            // The call to `read()` would return an error if the buffer was still in use by the GPU.
            let data_buffer_content = in_buffer.read().unwrap();
            let mut found_solution = false;
            for n in 0..100 {
                let mut s: String = data_buffer_content[n]
                    .into_iter()
                    .map(|a| (a + b'0') as char)
                    .collect();
                s.remove(14);
                s.remove(14);
                let num: usize = s.parse().unwrap();
                if num != 0 {
                    println!("{}: {}", n, num);
                    //found_solution = true;
                }
            }
            if found_solution {
                panic!("Found solution!");
            }
        }
        println!("Thing finished");

        panic!("Failed to find solution");

        todo!()
    }

    fn part2(&self, input: Input) -> Output {
        todo!()
    }
}

fn sub<const N: usize>(digits: &mut [u8; N], index: usize) {
    if digits[index] > 1 {
        digits[index] -= 1;
    } else {
        //We have to borrow
        if index == 0 {
            panic!("Would have negative result!");
        }
        sub::<N>(digits, index - 1);
        digits[index] = 9;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        let mut test: [u8; 3] = [2, 1, 1];
        sub(&mut test, 2);
        assert_eq!(test, [1, 9, 9]);

        let mut test: [u8; 3] = [2, 2, 2];
        sub(&mut test, 2);
        assert_eq!(test, [2, 2, 1]);

        let mut test: [u8; 3] = [2, 1, 1];
        sub(&mut test, 2);
        assert_eq!(test, [1, 9, 9]);
    }

    #[test]
    fn reduce() {
        let program = r#"inp y
inp y
mul y 0
inp z
add z y"#;
        let program = Program::parse(Input::new(program.to_string()));
        let (e, inputs) = program.build_side_effects(None);
        let expected = Program::parse(Input::new("inp z".to_string()));
        let mut expected = expected.build_side_effects(None).0;
        expected.index = 3;

        assert_eq!(e, expected);

        let expected: Vec<Vec<u8>> = vec![vec![0], vec![0], (1..10).into_iter().collect()];
        //assert_eq!(inputs, expected);
    }
}

impl Program {
    fn write_to_c_file(&self, name: &str, input_count: usize) -> std::io::Result<()> {
        let mut file = std::fs::File::create(name)?;
        writeln!(
            file,
            "{}",
            r#"
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

bool y2021_d24_execute(const uint8_t* input) {
     size_t w = 0;
     size_t x = 0;
     size_t y = 0;
     size_t z = 0;
"#
        )?;

        let mut input_index = 0;
        for ins in &self.0 {
            match ins {
                Ins::Inp(reg) => {
                    writeln!(file, "    {} = (size_t) input[{}];", reg, input_index)?;
                    input_index += 1;
                }
                Ins::Add(a, b) => writeln!(file, "    {} = {} + {};", a, a, b)?,
                Ins::Mul(a, b) => writeln!(file, "    {} = {} * {};", a, a, b)?,
                Ins::Div(a, b) => writeln!(file, "    {} = {} / {};", a, a, b)?,
                Ins::Mod(a, b) => writeln!(file, "    {} = {} % {};", a, a, b)?,
                Ins::Eql(a, b) => writeln!(
                    file,
                    "    if ({} == {}) {{ {} = 1; }} else {{ {} = 0; }};",
                    a, b, a, a
                )?,
            }
        }

        assert_eq!(input_count, input_index);

        writeln!(
            file,
            "{}",
            r#"
    return z == 0;
}
"#
        )?;

        Ok(())
    }

    /// Writes this program to a GLSL compute shader source
    /// `serials_per_core`: the amount of sequential serial numbers per core, must be a power of 10
    fn write_to_spv(&self, serials_per_core: usize, local_size: usize, out: &mut Vec<u8>) {

writeln!(
            out,
            r#"
#version 450

layout(local_size_x = {}, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {{
    uvec4 data[];
}} data;

void main() {{
    uint idx = gl_GlobalInvocationID.x;

    data.data[idx] = uvec4(0, 0, 0, 0);
}}
    "#,
            local_size
        )
        .unwrap();
        return;

        writeln!(
            out,
            r#"
#version 450

layout(local_size_x = {}, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0) buffer Data {{
    uvec4 data[];
}} data;

float real_floor(float x) {{
    return x + -sign(x) * abs(fract(x));
}}

uint pack_uints(uint a, uint b, uint c, uint d) {{
    return
        (a & 0xFF) << 24 |
        (b & 0xFF) << 16 |
        (c & 0xFF) <<  8 |
        (d & 0xFF) <<  0
    ;
}}

void main() {{
    uint idx = gl_GlobalInvocationID.x;
    uvec4 input_serial = data.data[idx];

    data.data[idx] = uvec4(0, 0, 0, 0);
    /*
    return;

    uint serial_digits[16];
    bool has_solution = false;
    uvec4 solution;

    serial_digits[ 0] = (input_serial.x >> 24) & 0xFF;
    serial_digits[ 1] = (input_serial.x >> 16) & 0xFF;
    serial_digits[ 2] = (input_serial.x >>  8) & 0xFF;
    serial_digits[ 3] = (input_serial.x >>  0) & 0xFF;

    serial_digits[ 4] = (input_serial.y >> 24) & 0xFF;
    serial_digits[ 5] = (input_serial.y >> 16) & 0xFF;
    serial_digits[ 6] = (input_serial.y >>  8) & 0xFF;
    serial_digits[ 7] = (input_serial.y >>  0) & 0xFF;

    serial_digits[ 8] = (input_serial.z >> 24) & 0xFF;
    serial_digits[ 9] = (input_serial.z >> 16) & 0xFF;
    serial_digits[10] = (input_serial.z >>  8) & 0xFF;
    serial_digits[11] = (input_serial.z >>  0) & 0xFF;

    serial_digits[12] = (input_serial.w >> 24) & 0xFF;
    serial_digits[13] = (input_serial.w >> 16) & 0xFF;
    serial_digits[14] = (input_serial.w >>  8) & 0xFF;
    serial_digits[15] = (input_serial.w >>  0) & 0xFF;
    "#,
            local_size
        )
        .unwrap();

        let serials_per_core_pow = f64::log10(serials_per_core as f64);
        assert_eq!(serials_per_core_pow % 1.0, 0.0);
        let serials_per_core_adjusted = 9usize.pow(serials_per_core_pow as u32);

        writeln!(
            out,
            r#"
    for (int i = 0; i < {}; i++) {{
        uint w = 0;
        uint x = 0;
        uint y = 0;
        uint z = 0;"#,
            serials_per_core_adjusted
        )
        .unwrap();

        let mut input_index = 0;
        for ins in &self.0 {
            match ins {
                Ins::Inp(reg) => {
                    writeln!(out, "        {} = serial_digits[{}];", reg, input_index).unwrap();
                    input_index += 1;
                }
                Ins::Add(a, b) => writeln!(out, "        {} = {} + {};", a, a, b).unwrap(),
                Ins::Mul(a, b) => writeln!(out, "        {} = {} * {};", a, a, b).unwrap(),
                Ins::Div(a, b) => writeln!(out, "        {} = {} / {};", a, a, b).unwrap(),
                Ins::Mod(a, b) => {
                    writeln!(out, "        {} = {} - {} * ({} / {});", a, a, b, a, b).unwrap()
                }
                Ins::Eql(a, b) => writeln!(out, "        {} = uint({} == {});", a, a, b).unwrap(),
            }
        }

        writeln!(
            out,
            r#"

        if (z == 0) {{
            solution = uvec4(
                pack_uints(serial_digits[ 0], serial_digits[ 1], serial_digits[ 2], serial_digits[ 3]),
                pack_uints(serial_digits[ 4], serial_digits[ 5], serial_digits[ 6], serial_digits[ 7]),
                pack_uints(serial_digits[ 8], serial_digits[ 9], serial_digits[10], serial_digits[11]),
                pack_uints(serial_digits[12], serial_digits[13], serial_digits[14], serial_digits[15])
            );
            //has_solution = true;
            break;
        }}
        //End of the main for loop. We need to decrement the serial
        for (int j = 13; j >= 0; j--) {{
            if (serial_digits[j] > 1) {{
                serial_digits[j] -= 1;
                break;
            }} else {{
                serial_digits[j] = 9;
            }}
        }}
    }}

    if (has_solution) {{
        data.data[idx] = solution;
    }} else {{
        data.data[idx] = uvec4(0, 0, 0, 0);
    }}
    data.data[idx] = uvec4(0, 0, 0, 0);
    */
}}"#
        )
        .unwrap();
    }
}

impl SideEffectTree {
    fn print_tree(&self) -> text_trees::StringTreeNode {
        let children = self.parents.iter().map(|(_reg, side)| side.print_tree());
        text_trees::StringTreeNode::with_child_nodes(self.ins.to_string(), children)
    }

    fn is_input(&self) -> Option<Register> {
        if let InstructionOr::Ins(Ins::Inp(reg)) = self.ins {
            Some(reg)
        } else {
            None
        }
    }

    fn reduce(mut self) -> Self {
        if let InstructionOr::Ins(Ins::Add(reg, operand)) = self.ins {
            if let Operand::Register(reg2) = operand {
                if let InstructionOr::KnownValue(0) = self.parents.get(&reg2).unwrap().ins {
                    let parent = self.parents.remove(&reg).unwrap();
                    //Anything plus a + 0 == a
                    self = parent;
                }
            }
        }
        let parents = std::mem::take(&mut self.parents);
        for (k, v) in parents {
            self.parents.insert(k, v.reduce());
        }
        self
    }
}
