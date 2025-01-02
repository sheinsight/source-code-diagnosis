import { Message } from "antd";
import { usePersistFn } from "ahooks";

export default function useHook() {
	const getLevelOptions = usePersistFn(() => {
		Message.error("error");
	});
	return null;
}
