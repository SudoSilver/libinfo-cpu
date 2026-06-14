## libinfo-cpu

This is a crate made to read some important data about the cpu.
It uses the `/proc/stat` and `/proc/cpuinfo` to pull:

1. Usage persentage
2. Flat usage in cpu ticks
3. CPU model 
4. Amount of logical threads
5. Amount of cores

You can access that data by using the `CpuInfo` struct.
The struct has the properties

1. `new()` to create the struct 
2. `update()` to update the data
3. `get_cpu_info()` which returns `[&str;3]` that contains the model then the amount of threads and then the amount of cores
4. `get_cpu_usage()` which returns `Option<(u8, u64)>` where it returns the usage% and flat usage

Note that `get_cpu_usage()` will return `None` if `update()` has not been run even once.

## Note 

Contribution and peer review is always welcome as long as it is respectful and has good intentions. 
If you spot any issues in the code please open an issue in github to bring it to contributors attention.
The crate is still in its early stages and not meant for production.