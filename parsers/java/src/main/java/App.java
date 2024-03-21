import java.util.ArrayList;
import java.util.HashMap;
import java.util.function.Function;

public class App {
    public static void main(String[] args) {
        // carracing
        {
            Function<ArrayList<Integer>, ExtractorOutput> extractor = meta -> {
                HashMap<String, Integer> parsed = new HashMap();
                parsed.put("track_length", meta.get(0));
                return new ExtractorOutput(meta.get(1), parsed);
            };

            ReplyInput _input = new ReplyInput("../test-data/input-carracing-b975.txt", extractor);
        }

        // slotmachine
        {
            Function<ArrayList<Integer>, ExtractorOutput> extractor = meta -> {
                HashMap<String, Integer> parsed = new HashMap();
                parsed.put("final_budget", meta.get(1));
                parsed.put("initial_budget", meta.get(2));
                return new ExtractorOutput(meta.get(0), parsed);
            };

            ReplyInput _input =
                    new ReplyInput("../test-data/input-slotmachine-4d99.txt", extractor);
        }

        // flowers
        {
            Function<ArrayList<Integer>, ExtractorOutput> extractor = meta -> {
                HashMap<String, Integer> parsed = new HashMap();
                parsed.put("W", meta.get(0));
                parsed.put("H", meta.get(1));
                parsed.put("F", meta.get(2));
                parsed.put("G", meta.get(3));
                return new ExtractorOutput(meta.get(3), parsed);
            };

            ReplyInput _input = new ReplyInput("../test-data/input-flowers-9c96.txt", extractor);
        }

        // save
        {
            ReplyOutput output = new ReplyOutput(new Object[] {1, 23, 456, 567});
            output.save("../test-data/java-output.txt");
        }
    }
}
