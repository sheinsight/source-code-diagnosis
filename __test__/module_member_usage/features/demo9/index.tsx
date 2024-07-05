import  * as antd from "antd"
import { deepClone } from "lodash"

function App(props) {

    useEffect(() => {
        let x = deepClone(props.user)
    },[])

    return (
        <div>
            <antd.Button />
        </div>
    )
}