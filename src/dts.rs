// Copyright (c) 2022, Michael Zhao
// SPDX-License-Identifier: MIT

pub struct Dts {}

#[cfg(test)]
mod tests {
    use crate::attribute::Attribute;
    use crate::element::Element;
    use crate::node::Node;
    use crate::tree::Tree;
    use std::sync::{Arc, Mutex};

    #[test]
    fn test_dts_0() {
        // Read the DTS text from test data folder
        let dts_0_text = std::fs::read_to_string("test/dts_0.dts").unwrap();
        println!("{dts_0_text}");
        // Build the same device tree with API and compare
        let mut root = Node::new("");
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("linux,dummy-virt")],
        ))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#address-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "interrupt-parent",
            1u32,
        ))));
        let dt = Tree::new(root);
        let dts = dt.to_dts(0);
        assert_eq!(dts_0_text, dts);
    }

    #[test]
    fn test_dts_1() {
        // Read the DTS text from test data folder
        let dts_1_text = std::fs::read_to_string("test/dts_1.dts").unwrap();
        println!("{dts_1_text}");
        // Build the same device tree with API and compare
        let mut root = Node::new("");
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#address-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 2u32))));
        let mut memory = Node::new("memory");
        memory.add_attr(Arc::new(Mutex::new(Attribute::new(
            "device_type",
            String::from("memory"),
        ))));
        let reg = vec![0u32, 0x40000000u32, 1u32, 0u32];
        memory.add_attr(Arc::new(Mutex::new(Attribute::new("reg", reg))));
        root.add_sub_node(memory);
        let dt = Tree::new(root);
        let dts = dt.to_dts(0);
        assert_eq!(dts_1_text, dts);
    }

    #[test]
    fn test_dts_2() {
        // Read the DTS text from test data folder
        let dts_2_text = std::fs::read_to_string("test/dts_2.dts").unwrap();
        println!("{dts_2_text}");

        // Build the same device tree with API and compare
        let mut root = Node::new("");
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("linux,dummy-virt")],
        ))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#address-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "interrupt-parent",
            1u32,
        ))));

        // CPUs
        let mut cpus = Node::new("cpus");
        cpus.add_attr(Arc::new(Mutex::new(Attribute::new("#address-cells", 2u32))));
        cpus.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 0u32))));

        // CPU@0
        let mut cpu0 = Node::new("cpu@0");
        cpu0.add_attr(Arc::new(Mutex::new(Attribute::new(
            "device_type",
            String::from("cpu"),
        ))));
        cpu0.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("arm,arm-v8")],
        ))));
        cpu0.add_attr(Arc::new(Mutex::new(Attribute::new(
            "enable-method",
            String::from("psci"),
        ))));
        let reg = vec![0u32, 0u32];
        cpu0.add_attr(Arc::new(Mutex::new(Attribute::new("reg", reg))));
        cpus.add_sub_node(cpu0);

        // CPU@1
        let mut cpu1 = Node::new("cpu@1");
        cpu1.add_attr(Arc::new(Mutex::new(Attribute::new(
            "device_type",
            String::from("cpu"),
        ))));
        cpu1.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("arm,arm-v8")],
        ))));
        cpu1.add_attr(Arc::new(Mutex::new(Attribute::new(
            "enable-method",
            String::from("psci"),
        ))));
        let reg = vec![0u32, 1u32];
        cpu1.add_attr(Arc::new(Mutex::new(Attribute::new("reg", reg))));
        cpus.add_sub_node(cpu1);

        root.add_sub_node(cpus);

        let dt = Tree::new(root);
        let dts = dt.to_dts(0);
        println!("{dts}");
        assert_eq!(dts_2_text, dts);
    }

    #[test]
    fn test_dts_3() {
        // Read the DTS text from test data folder
        let dts_3_text = std::fs::read_to_string("test/dts_3.dts").unwrap();
        println!("{dts_3_text}");

        // Build the same device tree with API and compare
        let mut root = Node::new("");
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("linux,dummy-virt")],
        ))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#address-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 2u32))));
        root.add_attr(Arc::new(Mutex::new(Attribute::new(
            "interrupt-parent",
            1u32,
        ))));

        // PCI
        let mut pci = Node::new("pci");
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "compatible",
            vec![String::from("pci-host-ecam-generic")],
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "device_type",
            String::from("pci"),
        ))));

        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "ranges",
            vec![
                0x2000000u32,
                0x0u32,
                0x10000000u32,
                0x0u32,
                0x10000000u32,
                0x0u32,
                0x20000000u32,
                0x3000000u32,
                0x1u32,
                0x40000000u32,
                0x1u32,
                0x40000000u32,
                0xfeu32,
                0xbfff0000u32,
            ],
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "bus-range",
            vec![0u32, 0u32],
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "#address-cells",
            0x3u32,
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new("#size-cells", 0x2u32))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "reg",
            vec![0u32, 0x30000000u32, 0x0u32, 0x10000000u32],
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "#interrupt-cells",
            1u32,
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new("interrupt-map", None))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new(
            "interrupt-map-mask",
            None,
        ))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new("dma-coherent", None))));
        pci.add_attr(Arc::new(Mutex::new(Attribute::new("msi-parent", 0x2u32))));

        root.add_sub_node(pci);

        let dt = Tree::new(root);
        let dts = dt.to_dts(0);
        println!("{dts}");
        assert_eq!(dts_3_text, dts);
    }
}