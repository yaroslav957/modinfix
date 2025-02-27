#include<linux/module.h>
#include<linux/kernel.h>

int init_module(void) {
    printk(KERN_INFO "Kmod Init\n");
    return 0;
}

void cleanup_module(void) {
    printk(KERN_INFO "kmod exit\n");
}
