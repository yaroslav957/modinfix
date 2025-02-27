#include<linux/module.h>
#include<linux/kernel.h>

// required
MODULE_LICENSE("GPL");
// optional
MODULE_AUTHOR("Yaroslav Lobachev");
MODULE_DESCRIPTION("My kernel module");
MODULE_VERSION("24.022022");


static int __init mod_init(void)
{
        printk(KERN_ALERT "KMod Init\n");
        return 0;
}

static void __exit mod_exit(void)
{
        printk(KERN_ALERT "KMod Exit\n");
}

static void any_void(void) {}
static void any_void2(void) {}

module_init(mod_init);
module_exit(mod_exit);