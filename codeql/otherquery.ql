import cpp

from Class c, Field f
    where c.getName() = "Buffer"
    and c.getAField() = f
select c.getName(), c.getSize(), c.getLocation(), f.getByteOffset(), f.getName()
