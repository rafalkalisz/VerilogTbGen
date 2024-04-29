# Verilog Testbench Generator
Tiny script I've written in Rust to automate writing the DUT instantiation part of a verilog testbench. Searches the provided verilog module file for any input/output ports and outputs a file with wires/regs for each one of them alongside a DUT template.

## Install
Release section has a built package for Linux/WSL with a basic bash script which moves the executable to ```usr\local\bin```.
```console
wget https://github.com/rafalkalisz/VerilogTbGen/releases/download/latest/verilog_tb_gen.tar.gz
tar -xvzf verilog_tb_gen.tar.gz
cd verilog_tb_gen
sudo ./install.sh
```
## Usage
Simply run the command to generate the testbench file (with _tb appended to the module name) in the working directory.
```console
verilogtbgen VerilogModule.v
```
