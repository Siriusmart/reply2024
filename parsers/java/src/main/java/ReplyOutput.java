import java.io.File;
import java.io.FileWriter;
import java.io.IOException;

public class ReplyOutput {
    Object[] data;

    public ReplyOutput(Object[] data) {
        this.data = data;
    }

    public void save(String path) {
        String[] out = new String[data.length];

        for (int i = 0; i < data.length; i++) {
            out[i] = String.format("Case #%d: %s", i + 1, data[i]);
        }

        String content = String.join("\n", out);

        try {
            File file = new File(path);
            file.delete();

            FileWriter writer = new FileWriter(path, true);
            writer.write(content);
            writer.close();
        } catch (IOException e) {
            e.printStackTrace();
            throw new RuntimeException("could not write to file");
        }
    }
}
