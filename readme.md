# Iterum Daemon

This repository contains the code with regards to the so-called daemon of Iterum. This software artifact contains the source code with regards to the data versioning server, and the storage interface of the Iterum architecture. 

# Setting up
 
A prebuild docker image for the daemon is present on [DockerHub](https://hub.docker.com/u/iterum). The daemon is one of the software artifacts of the Iterum framework, and requires the other software artifacts to be present in the cluster to function properly. A general overview of the Iterum framework can be found [here](https://github.com/iterum-provenance/iterum). 

The daemon can be deployed on a Kubernetes cluster using the instructions stated in the [cluster repository](https://github.com/iterum-provenance/cluster). 


# Code organization

The source code is split into three main modules:
* *dataset*, responsible for the data versioning server tasks
* *backend*, responsible for communicating with storage backends
* *pipeline*, responsible for the managing of results and provenance information of pipelines

These top-level modules are further split into submodules. This is further explained in the lower-level code documentation. The command `cargo doc --no-deps --open` can be run to compile the documentation for this project. The command opens up a browsable site where the code documentation for each module can be read.


# Building of the software artifact

The Daemon can be build to be used in a Kubernetes cluster by building the docker image. This can be done using the following command:

```
docker build -t daemon:<TAG> .
```
