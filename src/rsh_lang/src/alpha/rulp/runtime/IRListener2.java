package alpha.rulp.runtime;

import alpha.rulp.lang.IRObject;
import alpha.rulp.lang.RException;

public interface IRListener2<T1 extends IRObject, T2 extends IRObject> {
	public void doAction(T1 o1, T2 o2) throws RException;
}