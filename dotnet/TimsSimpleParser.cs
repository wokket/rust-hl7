using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace ConsoleApp1
{

    ///
    /// A naiive  .net core conversion of my rust code for basic perf comparison.
    public class TimsSimpleParser
    {

        private class Seperators
        {
            /// constant value, spec fixed to '\r' (ASCII 13, 0x0D)
            internal char Segment = '\r';
            internal char Field = '|';
            internal char Repeat = '~';
            internal char Component = '^';
            internal char Subcomponent = '&';
            internal char Escape_char = '\\';

            public Seperators() { }

            public Seperators(string msg)
            {
                Debug.Assert(msg[0] == 'M');
                Debug.Assert(msg[1] == 'S');
                Debug.Assert(msg[2] == 'H');

                Field = msg[3];
                Component = msg[4];
                Repeat = msg[5];
                Escape_char = msg[6];
                Subcomponent = msg[7];
            }
        }

        public class Repeat
        {
            public List<string> Components = new List<string>();
            public string GetAsString()
            {
                var delims = new Seperators();

                if (Components.Count == 0)
                {
                    return "";
                }
                else
                {
                    return Components.Aggregate((i, j) => i + delims.Component + j);
                }
            }
        }

        public class Field
        {
            public List<Repeat> Repeats = new List<Repeat>();

            public string GetAllAsString()
            {
                if (Repeats.Count == 0)
                {
                    return "";
                }

                return Repeats.Select(x => x.GetAsString())
                            .Aggregate((i, j) => i + "~" + j);
            }
        }

        public class Segment
        {
            public List<Field> Fields = new List<Field>();
        }
        public class Message
        {
            public List<Segment> Segments = new List<Segment>();
            public string Input;

            public Message(string msg)
            {
                Input = msg;
            }

            public void BuildFromInput()
            {
                var delims = new Seperators(Input);
                foreach (var segmentText in Input.Split(delims.Segment))
                {
                    if (String.IsNullOrEmpty(segmentText))
                    {
                        //end of message, go no further
                        break;
                    }

                    var segment = ParseSegment(segmentText, delims);
                    Segments.Add(segment);
                }
            }

            public string GetField(string segmentType, int field)
            {

                var segment = GetSegments(segmentType).First();
                return segment.Fields[field].GetAllAsString();
            }

            public IEnumerable<Segment> GetSegments(string segmentType)
            {
                foreach (var segment in Segments)
                {
                    var header = segment.Fields[0].GetAllAsString();
                    if (header == segmentType)
                    {
                        yield return segment;
                    }
                }
            }

            private Segment ParseSegment(string input, Seperators delims)
            {
                var fields = input.Trim()
                .Split(delims.Field)
                .Select(x => ParseField(x, delims));

                var segment = new Segment();
                segment.Fields.AddRange(fields);
                return segment;
            }

            private Field ParseField(string input, Seperators delims)
            {

                var returnValue = new Field();

                foreach (var repeatValue in GetRepeats(input, delims))
                {
                    var components = GetComponents(repeatValue, delims);

                    var repeat = new Repeat();
                    repeat.Components.AddRange(components);
                    returnValue.Repeats.Add(repeat);
                }

                return returnValue;
            }

            private IEnumerable<string> GetRepeats(string input, Seperators delims)
            {
                if (string.IsNullOrEmpty(input))
                {
                    yield break;
                }

                //can't yield break and return the whole enum in one, need to iterate :(

                foreach (var x in input.Split(delims.Repeat))
                {
                    yield return x;
                }
            }

            private IEnumerable<string> GetComponents(string input, Seperators delims)
            {
                if (string.IsNullOrEmpty(input))
                {
                    yield break;
                }

                //can't yield break and return the whole enum in one, need to iterate :(

                foreach (var x in input.Split(delims.Component))
                {
                    yield return x;
                }
            }
        }

        public Message Parse(string input)
        {
            var msg = new Message(input);
            msg.BuildFromInput();

            return msg;
        }

    }

}