// Copyright (c) 2016-2017 Chef Software Inc. and/or applicable contributors
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use hab_net::server::ZMQ_CONTEXT;
use protobuf::Message;
use protocol::jobsrv::{JobLogComplete, JobLogChunk};
use std;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::process;
use super::workspace::Workspace;
use zmq;

const INPROC_ADDR: &'static str = "inproc://logger";
const EOL_MARKER: &'static str = "\n";

/// ZMQ protocol frame to indicate a log line is being sent
const LOG_LINE: &'static str = "L";
/// ZMQ protocol frame to indicate a log has finished
const LOG_COMPLETE: &'static str = "C";


pub struct LogPipe {
    job_id: u64,
    sock: zmq::Socket,
}

impl LogPipe {
    pub fn new(workspace: &Workspace) -> Self {
        let sock = (**ZMQ_CONTEXT).as_mut().socket(zmq::PUSH).unwrap();
        sock.set_immediate(true).unwrap();
        sock.set_linger(5000).unwrap();
        sock.connect(INPROC_ADDR).unwrap();

        LogPipe {
            job_id: workspace.job.get_id(),
            sock: sock,
        }
    }

    /// Stream log output via ZMQ back to the Job Server for
    /// aggregation and streaming to downstream clients.
    ///
    /// Contents of STDOUT are streamed before any from STDERR (if
    /// any).
    pub fn pipe(&mut self, process: &mut process::Child) {
        let mut line_count = 0;
        // let file = match OpenOptions::new()
        //     .create(true)
        //     .read(true)
        //     .append(true)
        //     .open("/tmp/josh.log") {
        //     Ok(f) => f,
        //     Err(e) => panic!("CANT OPEN A FILE OMG, {:?}", e),
        // };

        if let Some(ref mut stdout) = process.stdout {
            let reader = BufReader::new(stdout);
            line_count = self.stream_lines(reader, line_count);
        }
        // writeln!(&file, "Just finished sending stdout to jobsrv");
        // writeln!(
        //     &mut std::io::stderr(),
        //     "Just finished sending stdout to jobsrv"
        // );
        if let Some(ref mut stderr) = process.stderr {
            let reader = BufReader::new(stderr);
            // not capturing line_count output because we don't use it
            self.stream_lines(reader, line_count);
        }
        // writeln!(&file, "Just finished sending stderr to jobsrv");
        // writeln!(
        //     &mut std::io::stderr(),
        //     "Just finished sending stderr to jobsrv"
        // );

        // Signal that the log is finished
        let mut complete = JobLogComplete::new();
        complete.set_job_id(self.job_id);
        self.sock.send_str(LOG_COMPLETE, zmq::SNDMORE).unwrap();
        self.sock
            .send(complete.write_to_bytes().unwrap().as_slice(), 0)
            .unwrap();
        // writeln!(&file, "Just signalled to jobsrv that the job is complete");
        // writeln!(
        //     &mut std::io::stderr(),
        //     "Just signalled to jobsrv that the job is complete"
        // );
    }

    /// Send the lines of the reader out over the ZMQ socket as
    /// `JobLogChunk` messages.
    ///
    /// `line_num` is the line number to start with when generating
    /// JobLogChunk messages. This allows us to send multiple output
    /// to the same job (i.e. standard output and standard error);
    /// send the first set using `line_num` = 0, send the second using
    /// whatever value the first invocation of `stream_lines`
    /// returned, etc.
    ///
    /// (I wrestled with the type system for an alternative
    /// implementation, but it defeated me :( This seems passable in
    /// the meantime.)
    fn stream_lines<B: BufRead>(&mut self, reader: B, mut line_num: u64) -> u64 {
        // let file = match OpenOptions::new()
        //     .create(true)
        //     .read(true)
        //     .append(true)
        //     .open("/tmp/josh.log") {
        //     Ok(f) => f,
        //     Err(e) => panic!("CANT OPEN A FILE OMG, {:?}", e),
        // };

        for line in reader.lines() {
            line_num = line_num + 1;
            let mut l: String = line.unwrap();
            l = l + EOL_MARKER;

            // writeln!(&file, "Sending |{}| to log forwarder", l.clone());
            // writeln!(
            //     &mut std::io::stderr(),
            //     "Sending |{}| to log forwarder",
            //     l.clone()
            // );

            let mut chunk = JobLogChunk::new();
            chunk.set_job_id(self.job_id);
            chunk.set_seq(line_num);
            chunk.set_content(l.clone());

            self.sock.send_str(LOG_LINE, zmq::SNDMORE).unwrap();
            self.sock
                .send(chunk.write_to_bytes().unwrap().as_slice(), 0)
                .unwrap();
        }
        line_num
    }
}
