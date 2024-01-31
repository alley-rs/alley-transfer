import Upload from "~/components/upload";
import "./index.scss";

const Send = () => {
  return (
    <div class="container" id="send">
      <div class="header">发送文件</div>
      <Upload action="/upload" />
    </div>
  );
};

export default Send;
