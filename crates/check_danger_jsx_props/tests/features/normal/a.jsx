import React, { useRef } from "react";

const DummyComponent = () => {
	const ref = useRef < HTMLInputElement > null;

	return <div data-if={true}>{Boolean(ref.current) ?? <input type="text" ref={ref} />}</div>;
};

export default DummyComponent;
