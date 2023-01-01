import cpp
import semmle.code.cpp.Print

from Class c
select getIdentityString(c), c.getSize()
