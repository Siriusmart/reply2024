import java.util.HashMap;

public class ExtractorOutput {
    public int case_len;
    public HashMap<String, Integer> meta;

    public ExtractorOutput(int case_len, HashMap<String, Integer> meta) {
        this.case_len = case_len;
        this.meta = meta;
    }
}
