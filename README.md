needed python modules: 
- cv2 (open cv)
- mediapipe (for hand recognition)

***
# <center>Silver Claw - Software Requirement and Design</center>

Version 1.00 

***
### About this document:
***
<div style="text-align: justify">
This document is based on [MIL STD 498] SRS and SDD documents which describe separatly requirements and design of modules.
The SRD document gathers requirements and design description of module. Not all sections of MIL STD 498 exist in this document because the level of detail required is not as important.
Sections 2 and 3 shall define functionning requirements and wide design decisions (SRS). Sections 4 and 5 describe architecture and details of software (SDD).
The SRD document shall centralize all informations about the modules which condition the software's design and ensure that it covers all identified requirements.
<br>
<br>
In details: <br>
- Section 2 shall contain all decisions which impact design and purpose of a module.
- Section 3 shar contain functional requirements of the module. Note that these requirements can be summarized in a set of functional scenarios provided by the module. To ensure traceability of these requirements, it is recommended to tag and reference them in a design section.
- Section 4 shall describe the architecture proposed to code the functional requirements listed in Section 3. Each component defined here shall reference the requirements' tags.
- Section 5 shall describe coding.
<br>
<br>
The level of detail is provided out of common sense. The more a description is detailled, the more the document will be subject to modifications.
<br>

- [<center>Silver Claw - Software Requirement and Design</center>](#centersilver-claw---software-requirement-and-designcenter)
    - [About this document:](#about-this-document)
- [Scope](#scope)
  - [Identification](#identification)
  - [System overview](#system-overview)
  - [Reference documents](#reference-documents)
- [Module wide design decisions](#module-wide-design-decisions)
- [Functional requirements](#functional-requirements)
  - [Required states and modes](#required-states-and-modes)
    - [Mouse mode](#mouse-mode)
    - [Sleep mode](#sleep-mode)
- [Module capability requirements](#module-capability-requirements)

***
# Scope
## Identification
## System overview
## Reference documents

# Module wide design decisions 
- 15/06/2022 - Global architecture (M.REMOND)<br>
    Silver Claw must be a service using the main camera to control the mouse on the main screen. This service is composed of two architectures:
    - A Python script using a Machine Learning module to detect the hand. (May be replaced by a C++ in the future.)
    - A Rust main program controlling the mouse accordingly.

    An installer should be available at some point.

# Functional requirements
## Required states and modes
***
- Silver Claw is a service which shall provide a way for the user to control its mouse with a camera. (SRD_SCLAW_001)<br>
- Silver Claw should provide a way to switch between modes with hand control. (SRD_SCLAW_002)<br>

### Mouse mode
This operating mode shall drive the mouse according to the user's hand movements and gestures.<br>

- This mode shall allow the user to left click. (SRD_SCLAW_101)<br>
- This mode shall allow the user to right click. (SRD_SCLAW_102)<br>
- This mode shall allow the user to scroll. (SRD_SCLAW_103) <br>
- This mode shall allow the user to move the mouse. (SRD_SCLAW_104)<br>
- This mode shall allow the user to switch to sleep. (SRD_SCLAW_105)<br>

### Sleep mode
This operating mode shall not control the mouse. It should only be possible to wake up the service. <br>

- This mode should take the least resources possible. (SRD_SCLAW_201)<br>
- This mode shall allow the user to wake up the program with an open hand gesture. (SRD_SCLAW_202)<br>

# Module capability requirements 