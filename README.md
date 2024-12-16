[![start with why](https://img.shields.io/badge/start%20with-why%3F-brightgreen.svg?style=flat)](https://www.ted.com/talks/simon_sinek_how_great_leaders_inspire_action)
[![HitCount](https://hits.dwyl.com/kpindur/mycoforge.svg?style=flat-square)](http://hits.dwyl.com/kpindur/mycoforge)
![GHA Build Status](https://github.com/kpindur/mycoforge/actions/workflows/rust.yml/badge.svg)
[![codecov](https://codecov.io/gh/kpindur/mycoforge/graph/badge.svg?token=ILPV0LKXBE)](https://codecov.io/gh/kpindur/mycoforge)

# MycoForge

MycoForge is a Rust (another) library designed to run genetic programming experiments. 
*(Hopefully)* It will provide a flexible set of tools and utilities to help researchers and practitioners implement, run, and analyze genetic programming algorithms with a focus on ease of use and extensibility. 

## Overview

Genetic programming is a method of evolving computer programs to solve complex problems using principles inspired by biological evolution. 
MycoForge aims to streamline the process of setting up genetic programming experiments, allowing users to focus on their specific research questions rather than implementation details.

Current directory tree (for visualization purposes):

src/  
├── common  
├── graph  
│   ├── core  
│   ├── fitness  
│   └── operators  
├── linear  
│   ├── core  
│   ├── fitness  
│   └── operators  
├── population  
├── tree  
│   ├── core  
│   ├── fitness  
│   └── operators  
└── utils  

Nothing for the time being

## Installation

NON EXISTENT

## Quick Start Guide

## Usage Examples

## Features

## Documentation

## Contributing Guidelines

## Contact Information

# Todo:
- [ ] README
    - [ ] Add outline
    - [ ] Add content
- [ ] Documentation
- [ ] Implementation
    - [ ] Primitive Set Management
        - [ ] protected mathematical operation (add, subtract, multiply, divide)
        - [ ] support for other mathematical functions (sin, cos, exp, log)
        - [ ] variable management for input Features
        - [ ] constant generation and management
        - [ ] function to validate primitive compatibility
    - [ ] TreeGenotype
        - [ ] simple tree structure
        - [ ] depth and size constraints?
        - [ ] methods for tree traversal
        - [ ] expression simplification?
        - [ ] tree validation
        - [ ] hard type trees?
    - [ ] Population Handling?
        - [ ] population initialization
            - [ ] Grow
            - [ ] Full
            - [ ] Ramped Half and Half
        - [ ] population size management
        - [ ] diversity maintenance mechanism?
        - [ ] population validation?
        - [ ] constraint checking?
        - [ ] cached evaluation
    - [ ] Genetic Operators
        - [ ] subtree crossover
        - [ ] mutations
            - [ ] point mutation
            - [ ] subtree mutation
            - [ ] constant mutation
            - [ ] ...?
        - [ ] operator probability management?
        - [ ] type safe genetic operations
        - [ ] operator result validation (or function correctness?)
    - [ ] Selection mechanism
        - [ ] tournament selection
        - [ ] fitness proportionate selection
        - [ ] age-based selection
        - [ ] multi-objective?
        - [ ] selection pressure adjustment
    - [ ] Fitness evaluation
        - [ ] mse
        - [ ] rmse
        - [ ] r-squared
        - [ ] multi-objective fitness options?
        - [ ] parallel fitness evaluation
        - [ ] fitness normalization
    - [ ] Optimizers
        - [ ] Evolutionary algorithms
            - [ ] generation control
            - [ ] termination criteria management
            - [ ] convergence detection
            - [ ] resource usage monitoring?
            - [ ] checkpointing?
        - [ ] EDA
            - [ ] linkage learning
            - [ ] distance metrics
            - [ ] distribution learning
        - [ ] Hyperparameter Management
            - [ ] population size
            - [ ] operator probability
            - [ ] selection pressure
            - [ ] tree size/depth constraints
            - [ ] runtime limits?
    - [ ] Analysis and visualization
        - [ ] Statistics tracking
            - [ ] best fitness
            - [ ] population diversity metrics
            - [ ] complexity measures
            - [ ] convergence metrics
            - [ ] resource usage (benchmarks and bottlenecks)?
        - [ ] Visualization tools
            - [ ] tree structure
            - [ ] fitness evolution
            - [ ] prediction vs actual plots
            - [ ] complexity evolution graphs
            - [ ] interactive visualizations?
    - [ ] Performance Features
        - [ ] Optimization
            - [ ] expression related?
            - [ ] parallel evaluation
            - [ ] memory management
            - [ ] caching system (possibly permanently stored store kind of thing?)
            - [ ] batch processing?
        - [ ] Scalability
            - [ ] distributed computing?
            - [ ] large dataset handling
            - [ ] memory efficiency?
            - [ ] progress monitoring
            - [ ] resource allocation and monitoring
    - [ ] User Interface
        - [ ] Configuration system
            - [ ] parameter validation (builder pattern?)
            - [ ] configuration file support?
            - [ ] command line interface?
            - [ ] runtime parameter adjustments?
            - [ ] default configuration (via default?)
            - [ ] logging system
        - [ ] Result Management
            - [ ] model serialization
            - [ ] result logging (logbook?)
            - [ ] export capabilities
            - [ ] version control
            - [ ] experiment tracking
    - [ ] Documentation
        - [ ] Code Documentation
            - [ ] function Documentation
            - [ ] struct Documentation
            - [ ] usage examples
            - [ ] performance tips
            - [ ] API refs?
        - [ ] User Guide
            - [ ] installation instructions
            - [ ] quick start guide
            - [ ] advanced usage scenario
            - [ ] best practices?
            - [ ] troubleshooting guide
    - [ ] Testing
        - [ ] Test suite
            - [ ] unit tests
            - [ ] integration tests
            - [ ] performance benchmarks
            - [ ] edge case handling
            - [ ] regression tests
            - [ ] comparison with other frameworks

