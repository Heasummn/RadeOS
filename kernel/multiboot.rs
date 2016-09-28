// This is taken from the multiboot 1.6 specification

/*          Mulitboot Information
    +-------------------+
    u32     | total_size        |
    u32     | reserved          |
    Followed by Tags            |
    +-------------------+
*/

pub unsafe fn load(address: usize) -> &'static MultiBootInfo
{
    &*(address as *const MultiBootInfo)
}

#[repr(C)]
pub struct MultiBootInfo
{
    pub total_size: u32,
    reserved: u32,
    tag: Tag // This is our first tag
}

// TODO: Split this up into a bigger module
impl MultiBootInfo
{  
    pub fn address_start(&self) -> usize 
    {
        self as *const _ as usize
    }

    pub fn address_end(&self) -> usize
    {
        self.address_start() + self.total_size as usize
    }

    pub fn iterator(&self) -> TagIterator
    {
        TagIterator { current: &self.tag as *const _ }
    }
}

// Each tag consists of a type, and a length

/*          Tag
    +-------------------+
    u32     | type              |
    u32     | size              |
    +-------------------+
*/

#[repr(C)]
pub struct Tag 
{
    pub typ: u32, // type is reserved TODO: remove public, only for testing
    size: u32 
    // Different fields depending on the type of Tag
}

/*
/*           Memory Info
    +-------------------+
    u32     | type = 4          |
    u32     | size = 16         |
    u32     | mem_lower         |
    u32     | mem_upper         |
    +-------------------+
*/
#[repr(C)]
pub struct MemInfo 
{
    typ: u32,
    size: u32,
    mem_lower: u32,
    mem_higher: u32
}

/*
             BIOS Boot Device
    +-------------------+
    u32     | type = 5          |
    u32     | size = 20         |
    u32     | biosdev           |
    u32     | partition         |
    u32     | sub_parition      |
    +-------------------+
*/

#[repr(C)]
pub struct BIOSBootDevice
{
    typ: u32,
    size: u32,
    biosdev: u32,
    partition: u32,
    sub_partition: u32
}
*/

pub const END_TAG: Tag = Tag { typ: 0, size: 8 };

// We can't use just a Tag, cause then we'd have to mutate it
pub struct TagIterator
{
    current: *const Tag
}

impl Iterator for TagIterator
{
    type Item = &'static Tag;

    fn next(&mut self) -> Option<&'static Tag>
    {
        match unsafe { &*self.current } 
        {
            &END_TAG    => None,
            tag         => 
            {
                // Get the current address
                let tag_addr = self.current as usize;
                
                // Goto the next address
                let next_addr = tag_addr + (tag.size as usize);
                
                // Align by 8
                let next_addr = ((next_addr - 1) & !0x7) + 0x8; // Cool hacks (not really)
                
                // Update the iterator!
                self.current = next_addr as *const _;
                
                // Return the current tag
                Some(tag)
            }   
        }
    }
}

#[macro_extern]
macro_rules! convert_tag
{
    (name:expr, current:expr) => 
        (current as *const Tag as *const $name)
}
