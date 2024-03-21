import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.HashMap;
import java.util.Scanner;
import java.util.function.Function;

public class ReplyInput {
    public ArrayList<ReplyCase> cases;

    public ReplyInput(String file, Function<ArrayList<Integer>, ExtractorOutput> meta_parser) {
        ArrayList<String> content = new ArrayList<>();
        try {
            File file_handle = new File(file);
            Scanner reader = new Scanner(file_handle);
            while (reader.hasNextLine()) {
                content.add(reader.nextLine());
            }
            reader.close();
        } catch (FileNotFoundException e) {
            e.printStackTrace();
            throw new RuntimeException("An error occurred when reading file");
        }

        int cases = Integer.parseInt(content.remove(0));

        ArrayList<ReplyCase> out = new ArrayList<>(cases);

        for (int case_no = 0; case_no < cases; case_no++) {
            if (content.isEmpty())
                throw new Error("reached EOF while there are still remaining cases");

            String[] meta_line = content.remove(0).split(" ");
            ArrayList<Integer> meta_cells = new ArrayList<>(meta_line.length);

            for (String meta : meta_line) {
                meta_cells.add(Integer.parseInt(meta));
            }
            ExtractorOutput extractor_output = meta_parser.apply(meta_cells);
            int case_len = extractor_output.case_len;
            HashMap<String, Integer> meta = extractor_output.meta;

            ArrayList<String[]> data = new ArrayList<>(cases);

            for (int case_len_no = 0; case_len_no < case_len; case_len_no++) {
                if (content.isEmpty())
                    throw new RuntimeException(
                            "reached EOF while there are still remaining entries");
                data.add(content.remove(0).split(" "));
            }

            if (data.size() != case_len) {
                throw new RuntimeException("expect " + case_len + " entries, got " + data.size());
            }

            out.add(new ReplyCase(meta, data));
        }

        if (out.size() != cases) {
            throw new RuntimeException("expect " + cases + " cases, got " + out.size());
        }

        this.cases = out;
    }

    @Override
    public String toString() {
        String case_viewer = "";

        for(ReplyCase case1: cases) {
            for(String line: case1.toString().split("\n")) {
                case_viewer += "\n    " + line;
            }

            case_viewer += ",";
        }

        return String.format("{\n    cases: [%s\n]}", case_viewer);
    }
}
