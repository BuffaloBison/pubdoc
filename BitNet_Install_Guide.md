# Microsoft BitNet (bitnet.cpp) -- Local Install Guide

> Official inference framework for 1-bit LLMs.
> Repo: <https://github.com/microsoft/BitNet>

---

## 1. Tool Requirements

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| **curl** | any | Download source archive from GitHub |
| **tar** | any | Extract downloaded archive |
| **CMake** | >= 3.22 | Build system for C++ compilation |
| **Clang** | >= 18 | C/C++ compiler (recommended over GCC) |
| **Python** | >= 3.9 | Run setup scripts and inference |
| **pip** | (bundled with Python) | Install Python dependencies |
| **Conda** | latest (recommended) | Manage isolated Python environment |
| **wget** | any | Used by the Clang install script |

### Hardware Recommendations

- **RAM**: 8 GB minimum (4 GB absolute minimum)
- **Disk**: ~10 GB free (source + model weights)
- **CPU**: x86_64 or ARM64
- **GPU**: Optional -- CUDA-capable NVIDIA GPU for GPU inference

---

## 2. Install Required Tools (Ubuntu/Debian)

Run these commands in order on a fresh machine.

### 2a. Update package lists and install basics

```bash
sudo apt-get update
sudo apt-get install -y curl wget tar build-essential
```

### 2b. Install CMake

```bash
sudo apt-get install -y cmake
```

Verify:

```bash
cmake --version
# Should show 3.22 or higher
```

If your distro ships an older CMake, install from Kitware's repo:

```bash
sudo apt-get install -y software-properties-common
wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc \
  | sudo apt-key add -
sudo apt-add-repository 'deb https://apt.kitware.com/ubuntu/ jammy main'
sudo apt-get update
sudo apt-get install -y cmake
```

### 2c. Install Clang (>= 18)

Use LLVM's automatic install script:

```bash
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"
```

Or install a specific version:

```bash
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
```

Verify:

```bash
clang --version
```

### 2d. Install Python 3.9+ and pip

```bash
sudo apt-get install -y python3 python3-pip python3-venv
```

Verify:

```bash
python3 --version
# Should show 3.9 or higher
```

### 2e. Install Conda (recommended)

```bash
curl -fsSL https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh \
  -o /tmp/miniconda.sh
bash /tmp/miniconda.sh -b -p $HOME/miniconda3
eval "$($HOME/miniconda3/bin/conda shell.bash hook)"
conda init
```

Close and reopen your terminal, then verify:

```bash
conda --version
```

---

## 3. Download BitNet Source (curl -- no git required)

Since git is not installed, use `curl` to download the source archive from GitHub:

```bash
curl -L https://github.com/microsoft/BitNet/archive/refs/heads/main.tar.gz \
  -o BitNet-main.tar.gz
```

Extract:

```bash
tar -xzf BitNet-main.tar.gz
cd BitNet-main
```

### Important: Download submodules manually

BitNet depends on the **llama.cpp** submodule. When downloading via `curl` (instead of `git clone --recursive`), you must fetch it separately:

```bash
curl -L https://github.com/ggerganov/llama.cpp/archive/refs/heads/master.tar.gz \
  -o llama-cpp.tar.gz
tar -xzf llama-cpp.tar.gz
rm -rf 3rdparty/llama.cpp
mv llama.cpp-master 3rdparty/llama.cpp
```

> **Alternative (if you install git later):** You can install git and clone normally:
> ```bash
> sudo apt-get install -y git
> git clone --recursive https://github.com/microsoft/BitNet.git
> cd BitNet
> ```

---

## 4. Build BitNet

### 4a. Create Conda environment and install Python dependencies

```bash
conda create -n bitnet-cpp python=3.9 -y
conda activate bitnet-cpp
pip install -r requirements.txt
```

### 4b. Download a model

Download the official BitNet-b1.58-2B-4T pre-quantized model from Hugging Face:

```bash
huggingface-cli download microsoft/BitNet-b1.58-2B-4T-gguf \
  --local-dir models/BitNet-b1.58-2B-4T
```

### 4c. Build and set up the environment

```bash
python setup_env.py -md models/BitNet-b1.58-2B-4T -q i2_s
```

This compiles the C++ inference engine via CMake and prepares the quantized model.

---

## 5. Run Inference

### Interactive conversation mode

```bash
python run_inference.py \
  -m models/BitNet-b1.58-2B-4T/ggml-model-i2_s.gguf \
  -p "You are a helpful assistant" \
  -cnv
```

### Single prompt

```bash
python run_inference.py \
  -m models/BitNet-b1.58-2B-4T/ggml-model-i2_s.gguf \
  -p "Daniel went back to the the the garden. Mary travelled to the kitchen. Where is Daniel?"
```

### Inference options

| Flag | Description | Default |
|------|-------------|---------|
| `-m` | Path to quantized GGUF model | (required) |
| `-p` | Prompt text | (required) |
| `-n` | Max tokens to generate | 128 |
| `-t` | Number of threads | system default |
| `-c` | Context size | 2048 |
| `-temp` | Temperature | 0.8 |
| `-cnv` | Enable conversation mode | off |

---

## 6. Other Supported Models

| Model | Parameters | Hugging Face Link |
|-------|-----------|-------------------|
| bitnet_b1_58-large | 0.7B | [1bitLLM/bitnet_b1_58-large](https://huggingface.co/1bitLLM/bitnet_b1_58-large) |
| bitnet_b1_58-3B | 3.3B | [1bitLLM/bitnet_b1_58-3B](https://huggingface.co/1bitLLM/bitnet_b1_58-3B) |
| Llama3-8B-1.58 | 8.0B | [HF1BitLLM/Llama3-8B-1.58-100B-tokens](https://huggingface.co/HF1BitLLM/Llama3-8B-1.58-100B-tokens) |
| Falcon3 Family | 1B-10B | [tiiuae/Falcon3](https://huggingface.co/collections/tiiuae/falcon3-67605ae03578be86e4e87026) |

To use a different model, substitute the `--hf-repo` or `--model-dir` flags in the build and inference commands.

---

## 7. Quick-Reference: All Commands in Order

```bash
# -- Install tools --
sudo apt-get update
sudo apt-get install -y curl wget tar build-essential cmake python3 python3-pip
bash -c "$(wget -O - https://apt.llvm.org/llvm.sh)"

# -- Install Conda --
curl -fsSL https://repo.anaconda.com/miniconda/Miniconda3-latest-Linux-x86_64.sh \
  -o /tmp/miniconda.sh
bash /tmp/miniconda.sh -b -p $HOME/miniconda3
eval "$($HOME/miniconda3/bin/conda shell.bash hook)"
conda init
# (restart shell here)

# -- Download BitNet source (no git needed) --
curl -L https://github.com/microsoft/BitNet/archive/refs/heads/main.tar.gz \
  -o BitNet-main.tar.gz
tar -xzf BitNet-main.tar.gz
cd BitNet-main

# -- Download llama.cpp submodule --
curl -L https://github.com/ggerganov/llama.cpp/archive/refs/heads/master.tar.gz \
  -o llama-cpp.tar.gz
tar -xzf llama-cpp.tar.gz
rm -rf 3rdparty/llama.cpp
mv llama.cpp-master 3rdparty/llama.cpp

# -- Set up Python env and build --
conda create -n bitnet-cpp python=3.9 -y
conda activate bitnet-cpp
pip install -r requirements.txt
huggingface-cli download microsoft/BitNet-b1.58-2B-4T-gguf \
  --local-dir models/BitNet-b1.58-2B-4T
python setup_env.py -md models/BitNet-b1.58-2B-4T -q i2_s

# -- Run --
python run_inference.py \
  -m models/BitNet-b1.58-2B-4T/ggml-model-i2_s.gguf \
  -p "You are a helpful assistant" -cnv
```

---

## References

- GitHub: <https://github.com/microsoft/BitNet>
- Technical Report: <https://arxiv.org/abs/2410.16144>
- Model on Hugging Face: <https://huggingface.co/microsoft/BitNet-b1.58-2B-4T>
- License: MIT
