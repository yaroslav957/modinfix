#include<linux/module.h>
#include<linux/kernel.h>

// required
MODULE_LICENSE("GPL");
// optional
MODULE_AUTHOR("Yaroslav Lobachev");
MODULE_DESCRIPTION("The most patriotic ZOV-module");
MODULE_VERSION("24.022022");


static int __init mod_init(void)
{
        printk(KERN_ALERT "ZellO ZOV VrOm ZOVnel !!! \n");
        return 0;
}

static void __exit mod_exit(void)
{
        printk(KERN_ALERT "Zexiting ZOVworld ZOV VrOm ZOVnel !!!\n");
}

module_init(mod_init);
module_exit(mod_exit);