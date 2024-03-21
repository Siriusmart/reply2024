import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashMap;

public class ReplyCase {
    public HashMap<String, Integer> meta;
    public ArrayList<String[]> data;

    public ReplyCase(HashMap<String, Integer> meta, ArrayList<String[]> data) {
        this.meta = meta;
        this.data = data;
    }

    @Override
    public String toString() {
        String arr_viewer = "";

        for(String[] row: data) {
            arr_viewer += "\n    " + Arrays.toString(row);
        }

        return String.format("{\n    meta: %s,\n    data: [%s\n]}", meta.toString(), arr_viewer);
    }
}
